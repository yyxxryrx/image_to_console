# image_to_console_core

> This is the core library of `image_to_console` project - a Rust library for converting images to terminal ASCII art, supporting multiple image formats and terminal output methods.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`image_to_console_core` is a Rust library that converts images into terminal-friendly formats including ASCII art and colored output. It supports various terminal protocols and image formats, making it easy to display images directly in the terminal.

## Features

- Multiple terminal protocols support:
  - Standard colored output (24-bit color)
  - Kitty graphics protocol
  - WezTerm graphics protocol
  - iTerm2 graphics protocol
  - Sixel graphics protocol (with `sixel` feature)
- Various image format support through the `image` crate
- Multiple display modes:
  - Full color with background/foreground colors
  - Half-block color mode
  - ASCII mode
  - No-color (grayscale) mode
- Image resizing with different algorithms
- GIF support (with `gif` feature)
- Parallel processing for better performance
- Compression options for more efficient output

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
image_to_console_core = "0.1"
```

Basic usage example:

```rust
use image_to_console_core::{DisplayMode, processor::{ImageProcessor, ImageProcessorOptions}};
use image::error::ImageResult;
fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;
    let options = ImageProcessorOptions {
        mode: DisplayMode::HalfColor,
        // ... other options
    };

    let processor = ImageProcessor::new(img, options);
    let result = processor.process();
    // result.lines contains the formatted terminal output
    Ok(())
}
```

## Features Flags

- `sixel` - Enable Sixel graphics protocol support
- `gif` - Enable GIF processing support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.