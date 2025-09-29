# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.16] - 2025-09-29

### Added

- Support center display mode in **Wezterm protocol**
- **Sixel protocol** can dithering control (`--disable-dither`),
- **Clear screen control** with `--clear` flag before/after rendering.
- Support for loading images from **standard input** via `image_to_console bytes`.
- **Auto terminal protocol detection** via new `--protocol auto` option using `crossterm`.
- **Output compression** with `--enable-compression` to reduce redundant ANSI sequences.
- Introduced `AudioPath` enum for robust audio file handling (temporary/custom paths with auto-cleanup).

### Changed

- Replaced `Option<String>` audio path with `AudioPath` enum.
- Renamed feature `rodio` to `audio_support` across the codebase and documentation.
- Updated `image_to_console` from `0.1.15` → `0.1.16`.
- Updated `image_to_console_core` from `0.1.16` → `0.1.17`.

### Fixed
- Fixed character rendering logic in non-full modes.
- Removed redundant `\x1b` in ANSI escape sequences.
- Fixed missing `sixel_support` feature dependency in `image-to-console-renderer`.
- Corrected `DisplayMode::is_full()` to properly recognize `SixelFull` as full-resolution.

### Refactor
- Refactored configuration parsing logic: extracted `Config` building into `from_cli`, added `Clone` to CLI structs, unified path handling.
- Replaced raw `u8` color index with `ColorIndexState` enum for safer state management.
- Split rendering logic into dedicated crates: `image-to-console-renderer` and `image-to-console-colored`.
-  Improved GIF/video player with on-demand audio loading.
- Upgraded project to Rust 2024 edition.

---

## [0.1.15] - 2025-09-15

### Added
- **Sixel protocol support**, quantization, and indexed image rendering.

### Changed
- Renamed CLI flag `--full-resolution` to `--half-resolution` (logic inverted for clarity).
- Adjusted frame delay calculation in Sixel mode for better playback smoothness.

### Fixed
- Fixed out-of-bounds access in Sixel rendering by using actual image dimensions.

### Refactor
- Optimized color handling with usage counting and parallel pixel processing.
- Improved GIF/video player with bounded channels

### Documentation
- Updated README (EN & ZH) with Sixel, auto-detection, stdin, compression, and audio support details.
- Marked Sixel as “implemented” in terminal compatibility tables.

---

## [0.1.14] - 2025-09-13

### Added
- **Video playback support** using `video-rs` (replacing `ffmpeg-next`), with custom audio track support.
- **GIF animation playback** with configurable FPS, looping, and frame buffering.

### Changed
- Made `gif` dependency optional via feature flag.

### Performance
- Dynamically sized frame channel based on actual video FPS.

### Documentation
- Added FFmpeg installation instructions per OS.
- Introduced feature dependency table in README.

---

## [0.1.13] - 2025-09-11

### Added
- CI support for video builds: added `libavcodec-dev`, `libavformat-dev`, `libavutil-dev` on Ubuntu; `ffmpeg` on macOS via Homebrew; `ffmpeg` via vcpkg on Windows.

### Refactor
- Restructured project into workspace: `image-to-console` (CLI) and `image_to_console-core` (library).

### Performance
- Multi-threaded GIF frame processing using `crossbeam-channel`.

### Changed

- Simplified release workflow to upload a single asset variant.
- Conditionally compiled video-related code under `cfg(feature = "video")`.

---

## [0.1.12] - 2025-08-31

### Added
- **Kitty terminal protocol** support (`--protocol kitty`).
- **iTerm2 protocol** support (`--protocol iterm2`).
- **WezTerm protocol** support with centering logic.
- Custom resize modes: fixed width/height or auto-fit to terminal.
- Audio playback during GIF/video via `rodio`.

### Changed
- Replaced boolean `--wezterm` flag with unified `--protocol <normal|wezterm|kitty|iterm2|auto>` enum.
- Deprecated `--no-resize` help text clarified as “(Only run in auto mode)”.

### Fixed
- Fixed missing rendering of last row/column in images.
- Improved error handling in terminal protocol detection.

### Documentation
- Added Chinese README (`README_zh-CN.md`).
- Updated crate metadata and dependency list.

---

## [Unreleased]

_Note: All changes prior to 0.1.12 are considered initial development and not individually tracked in this changelog._

---

> **Versioning Note**:  
> This project follows semantic versioning. Given the current `0.x.y` format, breaking changes may occur between minor versions until `1.0.0` is released.