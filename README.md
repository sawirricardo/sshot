# sshot

A fast, low-memory CLI tool for taking website screenshots. Built with Rust and Chrome DevTools Protocol.

## Install

```sh
curl -sSL https://raw.githubusercontent.com/sawirricardo/sshot/main/install.sh | sh
```

Or build from source:

```sh
cargo install --path .
```

**Requires Chrome or Chromium installed on your system.**

## Usage

```sh
sshot <url> [options]
```

### Examples

```sh
# Basic screenshot
sshot example.com

# JPEG with custom quality
sshot example.com -f jpeg -q 80

# WebP optimized for smallest file size
sshot example.com -f webp --optimized

# Full page capture
sshot example.com --full-page

# Mobile device emulation
sshot example.com --device iphone-15

# Capture a specific element
sshot example.com -s ".hero"

# Dark mode
sshot example.com --dark-mode

# Open Graph image (1200x630)
sshot example.com --og-image -o og.png

# Custom viewport with aspect ratio
sshot example.com -w 1200 -a 4:3

# PDF output
sshot example.com -f pdf -o page.pdf

# Custom output path
sshot example.com -o screenshots/home.png

# Wait for SPA to load
sshot myapp.com -d 3
```

## Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--width` | `-w` | Viewport width in pixels | 1920 |
| `--height` | `-H` | Viewport height in pixels | 1080 |
| `--aspect-ratio` | `-a` | Aspect ratio (e.g. "16:9") | |
| `--format` | `-f` | Output format: png, jpeg, webp, avif, pdf | png |
| `--quality` | `-q` | Quality for lossy formats (1-100) | 90 |
| `--full-page` | | Capture entire scrollable page | |
| `--delay` | `-d` | Seconds to wait before capture | 0 |
| `--device` | | Emulate a device viewport | |
| `--selector` | `-s` | Capture a specific CSS selector | |
| `--output` | `-o` | Output file path | auto |
| `--dark-mode` | | Emulate dark color scheme | |
| `--optimized` | | Optimize for smaller file size (slower) | |
| `--og-image` | | Use OG image dimensions (1200x630) | |

## Supported Devices

`iphone-15`, `iphone-15-pro-max`, `iphone-se`, `pixel-7`, `pixel-8`, `galaxy-s24`, `ipad-pro`, `ipad-air`, `macbook-pro-14`, `macbook-air-13`

## Output Formats

| Format | Notes |
|--------|-------|
| PNG | Default, lossless |
| JPEG | Lossy, respects `--quality` |
| WebP | Lossy (or lossless with `--optimized`) |
| AVIF | Lossy, best compression, slower encoding |
| PDF | Vector PDF with selectable text |

## License

MIT
