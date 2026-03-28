mod browser;
mod config;
mod convert;
mod devices;
mod output;

use anyhow::Result;
use clap::Parser;
use config::CaptureConfig;

#[derive(Parser)]
#[command(name = "sshot", version, about = "A fast CLI tool for taking website screenshots")]
struct Cli {
    /// The URL to screenshot
    url: String,

    /// Viewport width in pixels
    #[arg(short = 'w', long, default_value_t = 1920)]
    width: u32,

    /// Viewport height in pixels
    #[arg(short = 'H', long, default_value_t = 1080)]
    height: u32,

    /// Aspect ratio (e.g. "16:9") — calculates height from width
    #[arg(short = 'a', long)]
    aspect_ratio: Option<String>,

    /// Output format: png, jpeg, webp, avif, pdf
    #[arg(short = 'f', long, default_value = "png")]
    format: String,

    /// Quality for lossy formats (1-100)
    #[arg(short = 'q', long, default_value_t = 90)]
    quality: u8,

    /// Capture the entire scrollable page
    #[arg(long)]
    full_page: bool,

    /// Seconds to wait before capture
    #[arg(short = 'd', long, default_value_t = 0)]
    delay: u64,

    /// Emulate a device (e.g. "iphone-15", "pixel-7")
    #[arg(long)]
    device: Option<String>,

    /// Capture only a specific CSS selector
    #[arg(short = 's', long)]
    selector: Option<String>,

    /// Output file path (auto-generated if omitted)
    #[arg(short = 'o', long)]
    output: Option<String>,

    /// Emulate dark color scheme
    #[arg(long)]
    dark_mode: bool,

    /// Optimize output for smaller file size (slower)
    #[arg(long)]
    optimized: bool,

    /// Use Open Graph image dimensions (1200x630)
    #[arg(long)]
    og_image: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let (width, height) = if cli.og_image {
        (1200, 630)
    } else {
        (cli.width, cli.height)
    };

    let mut cfg = CaptureConfig::new(
        cli.url,
        width,
        height,
        cli.aspect_ratio,
        cli.format,
        cli.quality,
        cli.full_page,
        cli.delay,
        cli.device,
        cli.selector,
        cli.output,
        cli.dark_mode,
        cli.optimized,
    )?;

    cfg.resolve_aspect_ratio()?;
    cfg.resolve_device()?;

    let output_path = cfg.output_path();
    let data = browser::capture(&cfg).await?;
    let converted = convert::convert(&data, &cfg)?;

    std::fs::write(&output_path, &converted)?;

    let size = converted.len();
    let human_size = if size > 1_048_576 {
        format!("{:.1} MB", size as f64 / 1_048_576.0)
    } else {
        format!("{:.1} KB", size as f64 / 1024.0)
    };

    println!("Saved {} ({})", output_path, human_size);

    Ok(())
}
