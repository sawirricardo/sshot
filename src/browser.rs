use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chromiumoxide::Page;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::emulation::{
    MediaFeature, SetDeviceMetricsOverrideParams, SetEmulatedMediaParams,
    SetUserAgentOverrideParams,
};
use chromiumoxide::cdp::browser_protocol::page::{CaptureScreenshotFormat, PrintToPdfParams};
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;

use crate::config::{CaptureConfig, OutputFormat};

pub async fn capture(cfg: &CaptureConfig) -> Result<Vec<u8>> {
    let profile_dir = TempProfileDir::new()?;
    let browser_config = BrowserConfig::builder()
        .window_size(cfg.width, cfg.height)
        .user_data_dir(profile_dir.path())
        .arg("--disable-gpu")
        .arg("--no-sandbox")
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build browser config: {}", e))?;

    let (mut browser, mut handler) = Browser::launch(browser_config).await.map_err(|e| {
        anyhow::anyhow!(
            "failed to launch Chrome. Is Chrome/Chromium installed? Error: {}",
            e
        )
    })?;

    let handle = tokio::spawn(async move { while let Some(_event) = handler.next().await {} });

    let page = browser.new_page("about:blank").await?;

    apply_emulation(&page, cfg).await?;

    page.goto(&cfg.url).await?;
    page.wait_for_navigation().await?;

    if cfg.delay > 0 {
        tokio::time::sleep(Duration::from_secs(cfg.delay)).await;
    }

    let data = if cfg.format == OutputFormat::Pdf {
        capture_pdf(&page).await?
    } else if let Some(ref selector) = cfg.selector {
        capture_selector(&page, selector).await?
    } else if cfg.full_page {
        capture_full_page(&page).await?
    } else {
        capture_viewport(&page).await?
    };

    browser.close().await?;
    handle.await?;

    Ok(data)
}

struct TempProfileDir {
    path: PathBuf,
}

impl TempProfileDir {
    fn new() -> Result<Self> {
        let mut path = std::env::temp_dir();
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("failed to determine current time: {}", e))?
            .as_nanos();
        path.push(format!("sshot-chromium-profile-{pid}-{nanos}"));
        std::fs::create_dir(&path).map_err(|e| {
            anyhow::anyhow!(
                "failed to create Chrome profile dir '{}': {}",
                path.display(),
                e
            )
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempProfileDir {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_dir_all(&self.path) {
            eprintln!(
                "warning: failed to remove temp Chrome profile '{}': {}",
                self.path.display(),
                err
            );
        }
    }
}

async fn apply_emulation(page: &Page, cfg: &CaptureConfig) -> Result<()> {
    if cfg.device.is_some() {
        let metrics = SetDeviceMetricsOverrideParams::new(
            cfg.width as i64,
            cfg.height as i64,
            cfg.device_scale_factor.unwrap_or(1.0),
            cfg.is_mobile.unwrap_or(false),
        );
        page.execute(metrics).await?;

        if let Some(ref ua) = cfg.user_agent {
            let ua_params = SetUserAgentOverrideParams::new(ua);
            page.execute(ua_params).await?;
        }
    }

    if cfg.dark_mode {
        let mut media = SetEmulatedMediaParams::default();
        media.features = Some(vec![MediaFeature {
            name: "prefers-color-scheme".to_string(),
            value: "dark".to_string(),
        }]);
        page.execute(media).await?;
    }

    Ok(())
}

async fn capture_viewport(page: &Page) -> Result<Vec<u8>> {
    let params = ScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Png)
        .build();
    let data = page.screenshot(params).await?;
    Ok(data)
}

async fn capture_full_page(page: &Page) -> Result<Vec<u8>> {
    let params = ScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Png)
        .full_page(true)
        .build();
    let data = page.screenshot(params).await?;
    Ok(data)
}

async fn capture_selector(page: &Page, selector: &str) -> Result<Vec<u8>> {
    let element = page.find_element(selector).await.map_err(|_| {
        anyhow::anyhow!("element matching selector '{}' not found on page", selector)
    })?;

    let screenshot = element.screenshot(CaptureScreenshotFormat::Png).await?;
    Ok(screenshot)
}

async fn capture_pdf(page: &Page) -> Result<Vec<u8>> {
    let mut params = PrintToPdfParams::default();
    params.print_background = Some(true);
    params.prefer_css_page_size = Some(true);
    let data = page.pdf(params).await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::TempProfileDir;

    #[test]
    fn creates_unique_profile_dirs() {
        let first = TempProfileDir::new().unwrap();
        let second = TempProfileDir::new().unwrap();

        assert_ne!(first.path(), second.path());
        assert!(first.path().exists());
        assert!(second.path().exists());
    }
}
