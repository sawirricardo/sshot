use std::io::Cursor;

use anyhow::Result;
use image::ImageReader;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::config::{CaptureConfig, OutputFormat};

pub fn convert(png_data: &[u8], cfg: &CaptureConfig) -> Result<Vec<u8>> {
    match cfg.format {
        OutputFormat::Png if cfg.optimized => optimize_png(png_data),
        OutputFormat::Png => Ok(png_data.to_vec()),
        OutputFormat::Pdf => Ok(png_data.to_vec()),
        OutputFormat::Jpeg => to_jpeg(png_data, cfg.quality),
        OutputFormat::Webp => to_webp(png_data, cfg.quality, cfg.optimized),
        OutputFormat::Avif => to_avif(png_data, cfg.quality, cfg.optimized),
    }
}

fn optimize_png(png_data: &[u8]) -> Result<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()?
        .decode()?;

    let mut buf = Vec::new();
    let encoder =
        PngEncoder::new_with_quality(&mut buf, CompressionType::Best, FilterType::Adaptive);
    img.write_with_encoder(encoder)?;

    Ok(buf)
}

fn to_jpeg(png_data: &[u8], quality: u8) -> Result<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()?
        .decode()?;

    let mut buf = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    img.write_with_encoder(encoder)?;

    Ok(buf)
}

fn to_webp(png_data: &[u8], quality: u8, optimized: bool) -> Result<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()?
        .decode()?;

    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let encoder = webp::Encoder::from_rgba(&rgba, w, h);
    let mem = if optimized {
        // Lossless for maximum compression when optimized
        encoder.encode_lossless()
    } else {
        encoder.encode(quality as f32)
    };

    Ok(mem.to_vec())
}

fn to_avif(png_data: &[u8], quality: u8, optimized: bool) -> Result<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(png_data))
        .with_guessed_format()?
        .decode()?;

    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let pixels: Vec<rgb::RGBA8> = rgba
        .pixels()
        .map(|p| rgb::RGBA8 {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect();

    let img_ref = ravif::Img::new(&pixels[..], w as usize, h as usize);

    // speed 1 = slowest/best compression, speed 10 = fastest
    let speed = if optimized { 1 } else { 6 };

    let encoded = ravif::Encoder::new()
        .with_quality(quality as f32)
        .with_speed(speed)
        .encode_rgba(img_ref)?;

    Ok(encoded.avif_file)
}
