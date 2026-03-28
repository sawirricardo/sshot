use anyhow::{bail, Result};

use crate::devices;
use crate::output;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Png,
    Jpeg,
    Webp,
    Avif,
    Pdf,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "png" => Ok(Self::Png),
            "jpeg" | "jpg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::Webp),
            "avif" => Ok(Self::Avif),
            "pdf" => Ok(Self::Pdf),
            _ => bail!("unsupported format '{}'. Supported: png, jpeg, webp, avif, pdf", s),
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::Avif => "avif",
            Self::Pdf => "pdf",
        }
    }
}

#[derive(Debug)]
pub struct CaptureConfig {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: Option<String>,
    pub format: OutputFormat,
    pub quality: u8,
    pub full_page: bool,
    pub delay: u64,
    pub device: Option<String>,
    pub selector: Option<String>,
    pub output: Option<String>,
    pub dark_mode: bool,
    pub optimized: bool,
    pub user_agent: Option<String>,
    pub device_scale_factor: Option<f64>,
    pub is_mobile: Option<bool>,
    pub has_touch: Option<bool>,
}

impl CaptureConfig {
    pub fn new(
        url: String,
        width: u32,
        height: u32,
        aspect_ratio: Option<String>,
        format: String,
        quality: u8,
        full_page: bool,
        delay: u64,
        device: Option<String>,
        selector: Option<String>,
        output: Option<String>,
        dark_mode: bool,
        optimized: bool,
    ) -> Result<Self> {
        let url = normalize_url(&url)?;
        let format = OutputFormat::from_str(&format)?;

        if quality == 0 || quality > 100 {
            bail!("quality must be between 1 and 100");
        }

        if width == 0 || height == 0 {
            bail!("width and height must be greater than 0");
        }

        Ok(Self {
            url,
            width,
            height,
            aspect_ratio,
            format,
            quality,
            full_page,
            delay,
            device,
            selector,
            output,
            dark_mode,
            optimized,
            user_agent: None,
            device_scale_factor: None,
            is_mobile: None,
            has_touch: None,
        })
    }

    pub fn resolve_aspect_ratio(&mut self) -> Result<()> {
        let ratio = match &self.aspect_ratio {
            Some(r) => r.clone(),
            None => return Ok(()),
        };

        let parts: Vec<&str> = ratio.split(':').collect();
        if parts.len() != 2 {
            bail!("invalid aspect ratio '{}'. Expected format: W:H (e.g., 16:9, 4:3)", ratio);
        }

        let w: u32 = parts[0]
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid aspect ratio width in '{}'", ratio))?;
        let h: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid aspect ratio height in '{}'", ratio))?;

        if w == 0 || h == 0 {
            bail!("aspect ratio components must be greater than 0");
        }

        self.height = self.width * h / w;
        Ok(())
    }

    pub fn resolve_device(&mut self) -> Result<()> {
        let device_name = match &self.device {
            Some(d) => d.clone(),
            None => return Ok(()),
        };

        let preset = devices::find_device(&device_name)?;
        self.width = preset.width;
        self.height = preset.height;
        self.user_agent = Some(preset.user_agent.to_string());
        self.device_scale_factor = Some(preset.scale);
        self.is_mobile = Some(preset.is_mobile);
        self.has_touch = Some(preset.has_touch);

        Ok(())
    }

    pub fn output_path(&self) -> String {
        match &self.output {
            Some(path) => {
                if path.ends_with('/') || path.ends_with('\\') {
                    let filename = output::generate_filename(&self.url, &self.format);
                    format!("{}{}", path, filename)
                } else {
                    path.clone()
                }
            }
            None => output::generate_filename(&self.url, &self.format),
        }
    }
}

fn normalize_url(raw: &str) -> Result<String> {
    let url_str = if !raw.starts_with("http://") && !raw.starts_with("https://") {
        format!("https://{}", raw)
    } else {
        raw.to_string()
    };

    url::Url::parse(&url_str).map_err(|e| anyhow::anyhow!("invalid URL '{}': {}", raw, e))?;

    Ok(url_str)
}
