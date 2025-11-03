# image_to_console_core

> This is the core library of `image_to_console` project - a Rust library for converting images to terminal ASCII art,
> supporting multiple image formats and terminal output methods.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io Version](https://img.shields.io/crates/v/image_to_console_core)](https://crates.io/crates/image_to_console_core)

## Requirements

- Rust 1.85.0 or later (due to Rust 2024 edition)

## Overview

`image_to_console_core` is a Rust library that converts images into terminal-friendly formats including ASCII art and
colored output. It supports various terminal protocols and image formats, making it easy to display images directly in
the terminal.

## Features

- Multiple terminal protocols support:
    - Standard colored output (24-bit color)
    - Kitty graphics protocol
    - WezTerm graphics protocol
    - iTerm2 graphics protocol
    - Sixel graphics protocol (with `sixel` feature)

> **Note**: This library is primarily developed and tested on Windows. While Kitty and iTerm2 protocols are supported,
> they may require specific terminal emulators on Windows that fully support these protocols. For the best experience
> with
> these protocols, testing on macOS/Linux environments is recommended.

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

> you can see in [examples/basic-example.rs](examples/basic-example.rs)

```rust
use image::error::ImageResult;
use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;

    // Use default config
    let option = ImageProcessorOptions::default();

    let mut processor = ImageProcessor::new(img, option);
    let result = processor.process();
    // Exception handling (this is only shown, not handled, please refer to the actual use of the need)
    let result = result.expect("Process image failed");
    // result.lines contains the formatted terminal output
    // you also can use display method to print
    println!("{}", result.display());
    Ok(())
}
```

or more simply

> you can see in [examples/simple-example.rs](examples/simple-example.rs)

```rust
use image::error::ImageResult;
use image_to_console_core::processor::{ImageProcessorOptions, ImageProcessorOptionsCreate};

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;

    // Use default config and process
    let result = ImageProcessorOptions::default()
        .create_processor(img)
        .process()
        .unwrap();

    // result.lines contains the formatted terminal output
    // you also can use display method to print
    println!("{}", result.display());
    Ok(())
}
```

or
> you can see in [examples/simple-example2.rs](examples/simple-example2.rs)

```rust
use image::error::ImageResult;
use image_to_console_core::show_image;

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;
    // show image
    show_image!(img);
    Ok(())
}
```

## Features Flags

- `sixel` - Enable Sixel graphics protocol support
- `gif` - Enable GIF processing support
- `clap_support` - clap support for `Protocol`
- `auto_select` - Auto select protocol
- `all` - Enable all features

## Dependencies

| Crate                                                   | Version | License | Purpose                                  |
|---------------------------------------------------------|---------|---------|------------------------------------------|
| [base64](https://crates.io/crates/base64)               | 0.22.1  | MIT     | Base64 encoding                          |
| [clap](https://crates.io/crates/clap)                   | 4.5.50  | MIT     | Command line argument parsing (optional) |
| [crossterm](https://crates.io/crates/crossterm)         | 0.29.0  | MIT     | Terminal control (optional)              | 
| [gif](https://crates.io/crates/gif)                     | 0.13.3  | MIT     | GIF animation decoding (optional)        |
| [image](https://crates.io/crates/image)                 | 0.25.8  | MIT     | Image encoding/decoding and processing   |
| [nohash-hasher](https://crates.io/crates/nohash-hasher) | 0.2.0   | MIT     | Sixel Fast Hash (optional)               |
| [num_cpus](https://crates.io/crates/num_cpus)           | 1.17.0  | MIT     | Get logical CPU core count               |
| [quantette](https://crates.io/crates/quantette)         | 0.3.0   | MIT     | Sixel image quantization (optional)      |
| [rayon](https://crates.io/crates/rayon)                 | 1.11.0  | MIT     | Data parallel computing                  |
| [terminal_size](https://crates.io/crates/terminal_size) | 0.4.3   | MIT     | Detect terminal size                     |

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

**Note**: This library is primarily developed on Windows. We especially appreciate testing and feedback on macOS/Linux
for Kitty and iTerm2 protocols.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
