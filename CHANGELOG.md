# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- **Video Decoder Module**: Introduced the `video-decoder` crate with frame extraction, error handling, and lifetime-aware decoding capabilities [#c895c4b, #469e647, #204ecf5, #73255d3]
- Integrated `video-decoder` as an optional dependency in the CLI crate with `video_player` feature [#04dd33d]
- **Processor Feature Support**: Added `processor` feature across workspace crates with updated dependency configuration [#ce25843]
- **Full Development Shell**: Added complete development shell configuration with updated toolchain dependencies [#37c59bd]
- Added `ToImageFailed` error variant for improved video decoding error handling [#469e647]
- Updated Nix Flake configuration with additional build packages and subdirectory build support [#db088f6, #a852950]
- **Video Playback Sync**: Implemented video playback synchronization functionality [#5c7b98e]

### Fixed

- Fixed URL parameter passing error in `dot_file` command [#0ed8dc4]
- Fixed video decoder error handling and frame extraction logic [#7edad71]
- Fixed frame rate fetching and frame data passing issues in video playback [#c25aac5]
- Unified error handling types in utility module [#87d2b9d]
- Enhanced Kitty terminal protocol recognition [#84b72f5]

### Performance

- Optimized frame synchronization logic in runner to improve playback performance [#206bf43]

### Refactor

- Extracted Sixel conversion logic into dedicated module `converter/sixel` [#d2763ed]
- Extracted Unicode conversion logic into dedicated module `converter/unicode` [#52c8af8]
- Restructured `VideoDecoder` with lifetime parameters and improved implementation [#204ecf5, #73255d3]
- Replaced manual `Drop` implementation with `Default` derive for `AudioPath` [#be58916]
- Updated function signatures and improved code style in `summon-schema-derive` [#4b9cf89]
- Removed unnecessary dereference in file path joining [#be0c836]
- Reordered imports and simplified display implementation in `image-to-console-colored` [#6a5c945]
- Removed redundant blank lines and optimized code structure [#39283ae]
- Restructured `video-decoder` to improve frame processing pipeline [#6b29cee]
- Changed video frame timestamp type from `i64` to `Duration` for better type safety [#6732a04]
- Removed unused dependencies and updated video decoding in CLI crate [#2c52845]
- Moved macro definitions to independent module in core library [#94cc6f3]
- Refactored video playback logic to support audio functionality [#e93677c]
- Extracted video and GIF processing logic into independent functions [#5ea7881]
- Optimized GIF processing logic and improved error handling [#3ec5683]
- Refactored configuration structure to separate image type parameters [#97eca4e]
- Refactored renderer code to remove unused parameters and conditional compilation [#c9bbd2e]
- Optimized string formatting and method naming [#eca71a5]
- Updated formatting string syntax [#aa5f150]
- Optimized packet processing logic in video decoder [#9d91b63]
- Added clippy lint rule to allow collapsible if [#bd964b0]

### Documentation

- Updated CLI parameters and configuration structs documentation comments [#3d7be6a]

### Dependencies

- Updated project dependencies to latest compatible versions [#b5ee3c0]

## [v0.2.0] - 2026-03-22

### Added

- Introduce the `build-options` procedural macro package, providing the `Options` derive macro for chained invocation of
  build configuration
- Added a new function to control the refresh interval for video playback, supporting modes such as seconds, frames,
  always, and never refresh
- Add the `-F/--flush-interval` parameter for the video playback command, with a default value of "1s"
- Add a `flush_interval` field to the `Config` struct to control the video refresh frequency
- **Nix Build Support**: Integrated `rust-overlay` for Rust toolchain management, configured `bindgenHook` for C library bindings, added support for Linux/Darwin multi-platform architectures, and pre-installed `rust-analyzer` and `clippy` in the dev shell [#3d67f93].
- **TOML Configuration System**: Introduced the `dot-file` subcommand supporting `schema`, `run`, and `check` operations. Implemented bidirectional conversion between TOML config and CLI arguments [#a8a13e7, #0037fb7].
- **Terminal Styling Module**: Added a new `styles` module implementing `Display` traits for `TextHeader` and `Text`, enabling unified formatting and custom style settings [#2e1a4fc].
- **Unicode Width Support**: Added `unicode-width` dependency to ensure correct alignment of multi-byte characters in console output [#69d2032].
- **New `summon-schema` crate**: Added a library for converting Rust types to JSON Schema with the `ToSchema` trait (#5ee1402, #4982c22)

### Changed

- Refactor the configuration building method, replacing the original `from_cli` static method with the `From<&Cli>`
  trait
  
- Change the configuration building pattern to chained calls to improve code readability and maintainability

- Enhance the robustness of `FlushInterval` parsing by adding validation for negative and zero values

- Update the video renderer to use a configurable refresh interval instead of the fixed 2-second refresh mechanism

- Optimize the `build-options` macro to correctly handle field visibility

- **HTTP Client Migration**: Replaced `reqwest` with `ureq` to simplify the dependency tree, reduce binary size, and improve compile times [#232de16].

- **FFmpeg Backend Upgrade**:
    - Replaced `ez-ffmpeg` with `ffmpeg-next` for audio extraction.
    
    - Upgraded dependencies: `bindgen` (0.70.1→0.72.1), `ffmpeg-next` (7.1.0→8.0.0), `video-rs` (0.10.5→0.11.0), `ndarray` (0.16.1→0.17.2), and others [#a3bdc58].
    
- **Config Module Refactor**: Moved `Cli` struct and command definitions into a dedicated `cli` module to improve maintainability [#c535076].

- **Error Handling Improvements**: Enhanced TOML parsing errors to display visual highlights with precise line and column numbers [#ef3d74c].

- **Serialization Convention**: Unified configuration structs to use `kebab-case` naming convention [#0037fb7].

### Fixed

- Fix the `Default` implementation of `ImageType` and provide a reasonable default value
- Improve the error handling mechanism to ensure that invalid refresh interval parameters are correctly rejected
- Fixed Unicode character width calculation in source code highlighting to prevent console misalignment [#69d2032].
- Fixed temporary audio file cleanup logic by adding `unwrap_or_default()` to prevent errors when files are missing [#ce480d6, #0037fb7].
- Fixed `AudioPath::drop` implementation to better handle filesystem permissions and concurrent access issues [#0037fb7].

### Refactor

- Use `eq_ignore_ascii_case` instead of `to_lowercase` for case-insensitive comparisons

### Removed

- Removed `reqwest` and its transitive dependencies: `addr2line`, `atomic-waker`, `backtrace`, `tokio`, etc. [#232de16].
- Removed legacy FFmpeg bindings: `ez-ffmpeg`, `core-foundation`, `crossbeam` [#a3bdc58].
- Removed `RunType::DotFile` enum variant in favor of the new subcommand pattern [#0037fb7].

### Documentation

- Updated README: Added TOML configuration examples, `dot-file` subcommand usage, and protocol support table [#a525ffe, #22c0809].
- Corrected video feature name: `video` → `video_player` and updated dependency table format [#22c0809].
- Improved Chinese documentation layout and refined parameter descriptions [#a525ffe].

### Internal

- Optimized string concatenation performance using the `write!` macro [#2e1a4fc].
- Added conditional compilation for audio support fields in GIF parameters to support builds [#0037fb7].
- Marked unimplemented `schema` commands explicitly with the `todo!` macro [#01384ee].

## [v0.1.18] - 2025-10-30

### Added

- Add `drop` trait for `AudioPath` to clean temporary audio files automatically.

### Changed

- Remove the clear temporary audio file logic in the end of the video playback logic.
- Change the delay calculation method to dynamic calculation not based on the fixed number.

### Refactor

- Replace the `if let` branch with the `get_path` method when init the audio mixer in the GIF playback logic
- Move the `Protocol` enum to the core library

## [v0.1.17] - 2025-10-24

### Added

- Add test, docs and examples for core library
- Add python bindings for core library
- Support ASCII mode

### Fixed

- Fix the file name processing logic

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

- Refactored configuration parsing logic: extracted `Config` building into `from_cli`, added `Clone` to CLI structs,
  unified path handling.
- Replaced raw `u8` color index with `ColorIndexState` enum for safer state management.
- Split rendering logic into dedicated crates: `image-to-console-renderer` and `image-to-console-colored`.
- Improved GIF/video player with on-demand audio loading.
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

- Added `FFmpeg` installation instructions per OS.
- Introduced feature dependency table in README.

---

## [0.1.13] - 2025-09-11

### Added

- CI support for video builds: added `libavcodec-dev`, `libavformat-dev`, `libavutil-dev` on Ubuntu; `ffmpeg` on macOS
  via Homebrew; `ffmpeg` via vcpkg on Windows.

### Refactor

- Restructured project into workspace: `image-to-console` (CLI) and `image-to-console-core` (library).

### Performance

- Multithreaded GIF frame processing using `crossbeam-channel`.

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

- Replaced Boolean `--wezterm` flag with unified `--protocol <normal|wezterm|kitty|iterm2|auto>` enum.
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
> This project follows semantic versioning. Given the current `0.x.y` format, breaking changes may occur between minor
> versions until `1.0.0` is released.

