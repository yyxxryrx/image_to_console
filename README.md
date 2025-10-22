# image_to_console

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Last Version](https://img.shields.io/github/v/release/yyxxryrx/image_to_console)](https://github.com/yyxxryrx/image_to_console/releases/latest) [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/yyxxryrx/image_to_console)

A high-performance terminal image viewer written in Rust.

English | [简体中文](README_zh-CN.md)

## Features

- 🖼️ **Multiple Sources**: Load images from files, URLs, Base64 strings and standard input
- 📁 **Directory Processing**: Batch process entire directories of image files
- 🎨 **True Color Support**: Display colored images in supported terminals
- ⚫ **Grayscale Mode**: Convert color images to grayscale character art
- ⚫ **Black Background**: Black background display option in grayscale mode
- ⚡ **Parallel Processing**: Fast conversion using Rayon library for parallel computation
- 📏 **Adaptive Sizing**: Automatically scale images to fit terminal window
- ⌨️ **Interactive Pause**: Optional pause after display for convenient viewing
- 🖥️ **Multiple Display Modes**: Support for full-resolution (default) and half-resolution display modes
- 🖥️ **Multiple Terminal Protocol Support**: Support for WezTerm, Kitty, iTerm2 and Sixel image protocols
- 🎞️ **GIF Animation Support**: Play GIF animations in terminal
- 🎥 **Video Playback Support**: Play video files in terminal (requires `video_player` feature and FFmpeg)
- 🔊 **Audio Support**: Add audio tracks to GIF animations (requires `audio_support` feature)
- 🗜️ **Compression Support**: Compress output in normal protocol mode

## Supported Protocols
- [x] WezTerm
- [x] Kitty
- [x] iTerm2
- [x] Sixel

## Features 
| Feature       | Dependencies                                       | Description                                                      | Is Default                          |
|---------------|----------------------------------------------------|------------------------------------------------------------------|-------------------------------------|
| reqwest       | reqwest                                            | HTTP client library                                              | <span style="color: green">✓</span> |
| audio_support | rodio                                              | Play audio support                                               | <span style="color: red">✗</span>   |
| crossterm     | crossterm                                          | Cross-platform terminal library                                  | <span style="color: green">✓</span> |
| gif_player    | gif, crossbeam-channel                             | Play GIF animations in terminal                                  | <span style="color: green">✓</span> |
| video_player  | ez-ffmpeg, video-rs, crossbeam-channel, **FFmpeg** | Play video files in terminal (requires video feature and FFmpeg) | <span style="color: red">✗</span>   |
| sixel_support | quantette, nohash-hasher                           | Display images using Sixel protocol                              | <span style="color: green">✓</span> |

## Installation

### Download from Release

1. Visit the [GitHub Release page](https://github.com/yyxxryrx/image_to_console/releases)
2. Download the pre-compiled binary for your operating system
3. Extract and add the binary to your system PATH

### Build from Source

```bash
git clone https://github.com/yyxxryrx/image_to_console.git
cd image_to_console

# Build basic version (without video support)
# No FFmpeg required for this version
cargo build --release

# Build version with video support
# FFmpeg libraries required for this version
cargo build --release --features video_player
```

The compiled binary will be located at `target/release/image_to_console`.

If you want to build with video support, you need to install FFmpeg libraries first:
- Ubuntu/Debian: `sudo apt-get install libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev`
- macOS: `brew install ffmpeg`
- Windows: Install FFmpeg and ensure DLL files are in your PATH

## Usage

### Basic Usage

```bash
# Load image from file
image_to_console file path/to/image.jpg

# Load all images from directory
image_to_console directory path/to/directory

# Load animation from GIF file
image_to_console gif path/to/animation.gif

# Load image from URL
image_to_console url https://example.com/image.png

# Load image from Base64 string
image_to_console base64 <base64-encoded-image-data>

# Load image from standard input bytes
cat image.jpg | image_to_console bytes

# Load video from file (requires video feature)
image_to_console video path/to/video.mp4
```

### Command Line Options

```bash
# Show help information
image_to_console --help

# Center display image
image_to_console -c file image.jpg

# Pause and wait for user input after display
image_to_console --pause file image.jpg

# Show processing time
image_to_console -t file image.jpg

# Half resolution display
image_to_console --half-resolution file image.jpg

# Grayscale mode display
image_to_console --no-color file image.jpg

# Use black background in grayscale mode
image_to_console --no-color -b file image.jpg

# Disable width scaling
image_to_console --without-resize-width file image.jpg

# Disable height scaling
image_to_console --without-resize-height file image.jpg

# Save output to file
image_to_console -o output.txt file image.jpg

# Disable image printing (show info only)
image_to_console --disable-print file image.jpg

# Disable info display
image_to_console --disable-info file image.jpg

# Read all images at once in directory mode
image_to_console --read-all directory path/to/directory

# Display image using WezTerm image protocol
image_to_console --protocol wezterm file image.jpg

# Display image using Kitty image protocol
image_to_console --protocol kitty file image.jpg

# Display image using iTerm2 image protocol
image_to_console --protocol iterm2 file image.jpg

# Display image using Sixel protocol
image_to_console --protocol sixel file image.jpg

# Display image using Sixel protocol with custom max colors
image_to_console --protocol sixel --max-colors 128 file image.jpg

# Enable compression (only in normal protocol)
image_to_console --enable-compression file image.jpg

# Auto detect terminal protocol (default)
image_to_console --protocol auto file image.jpg

```

### File Subcommand Options

```bash
# Hide filename display
image_to_console file --hide-filename image.jpg

# Show filename (default)
image_to_console file image.jpg
```

### Directory Subcommand Options

```bash
# Process all images in directory
image_to_console directory path/to/directory

# Process all images in directory and read all at once
image_to_console --read-all directory path/to/directory
```

### GIF Subcommand Options
> **requires `gif_player` feature**
> 
> **audio support requires `audio_support` feature**

```bash
# Play GIF animation (default 10 FPS)
image_to_console gif animation.gif

# Play GIF animation with custom frame rate
image_to_console gif --fps 24 animation.gif

# Loop GIF animation
image_to_console gif --loop animation.gif

# Set frame rate and loop GIF animation
image_to_console gif --fps 30 --loop animation.gif

# Play GIF animation with audio (requires audio_support feature)
image_to_console gif --audio audio.mp3 animation.gif
```

### Video Subcommand Options 
> **requires `video_player` feature**

```bash
# Play video file (requires video_player feature)
image_to_console video path/to/video.mp4

# Play video file with custom audio track (requires video_player feature)
image_to_console video --audio path/to/audio.mp3 path/to/video.mp4
```

## Display Mode Description

### Color Modes
- **Full-resolution color mode** (default): Display using upper/lower half-blocks, each character represents two pixels
- **Half-resolution color mode** (--half-resolution): Display using background color blocks, each character represents one pixel
- **WezTerm mode** (--protocol wezterm): Use WezTerm's inline image protocol to display original image directly
- **Kitty mode** (--protocol kitty): Use Kitty's inline image protocol to display original image directly
- **iTerm2 mode** (--protocol iterm2): Use iTerm2's inline image protocol to display original image directly
- **Sixel mode** (--protocol sixel): Use Sixel protocol to display images in supported terminals
- **Auto-detect mode** (--protocol auto): Automatically detect and use the best available terminal protocol (default)

### Grayscale Modes
- **Grayscale mode** (--no-color): Convert image to grayscale character art display
- Use different Unicode characters (such as █, ▀, ▄, ., , etc.) to represent different grayscale levels
- **Black background** (-b): Use black background to display image in grayscale mode
- **WezTerm grayscale mode** (--protocol wezterm --no-color): Display grayscale image in WezTerm
- **Kitty grayscale mode** (--protocol kitty --no-color): Display grayscale image in Kitty
- **iTerm2 grayscale mode** (--protocol iterm2 --no-color): Display grayscale image in iTerm2
- **Auto-detect grayscale mode** (--protocol auto --no-color): Automatically detect and use the best available terminal protocol in grayscale mode

## Supported Image Formats

Supports most common image formats, including but not limited to:
- JPEG
- PNG
- GIF
- BMP
- ICO
- TIFF
- WebP

## Dependencies

| Crate                                                           | Version | License          | Purpose                                 |
|-----------------------------------------------------------------|---------|------------------|-----------------------------------------|
| [clap](https://crates.io/crates/clap)                           | 4.5.20  | MIT / Apache-2.0 | Command line argument parsing           |
| [rayon](https://crates.io/crates/rayon)                         | 1.11.0  | MIT / Apache-2.0 | Data parallel computing                 |
| [num_cpus](https://crates.io/crates/num_cpus)                   | 1.17.0  | MIT              | Get logical CPU core count              |
| [image](https://crates.io/crates/image)                         | 0.25.4  | MIT              | Image encoding/decoding and processing  |
| [base64](https://crates.io/crates/base64)                       | 0.22.1  | MIT / Apache-2.0 | Base64 encoding/decoding                |
| [indicatif](https://crates.io/crates/indicatif)                 | 0.17.8  | MIT              | Terminal progress bars                  |
| [terminal_size](https://crates.io/crates/terminal_size)         | 0.4.0   | MIT              | Detect terminal size                    |
| [crossterm](https://crates.io/crates/crossterm)                 | 0.29.0  | MIT              | Terminal control and input (optional)   |
| [reqwest](https://crates.io/crates/reqwest)                     | 0.12.9  | MIT / Apache-2.0 | Blocking HTTP client   (optional)       |
| [gif](https://crates.io/crates/gif)                             | 0.13.3  | MIT              | GIF animation decoding (optional)       |
| [crossbeam-channel](https://crates.io/crates/crossbeam-channel) | 0.5.15  | MIT / Apache-2.0 | Cross-thread communication   (optional) |
| [rodio](https://crates.io/crates/rodio)                         | 0.21.1  | MIT / Apache-2.0 | Audio playback (optional)               |
| [ez-ffmpeg](https://crates.io/crates/ez-ffmpeg)                 | 0.5.3   | MIT              | Video processing (optional)             |
| [video-rs](https://crates.io/crates/video-rs)                   | 0.10.3  | MIT              | Video processing (optional)             |
| [ndarray](https://crates.io/crates/ndarray)                     | 0.16.1  | MIT              | N-dimensional array (optional)          |
| [quantette](https://crates.io/crates/quantette)                 | 0.3.0   | MIT              | Sixel image quantization (optional)     |
| [nohash-hasher](https://crates.io/crates/nohash-hasher)         | 0.2.0   | MIT              | Sixel Fast Hash (Optional)              |
| [pyo3](https://crates.io/crates/pyo3)                           | 0.27.1  | MIT / Apache-2.0 | Python bindings (optional)              |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.