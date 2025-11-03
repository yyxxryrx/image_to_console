# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- Add `error` module
    - Add `ConvertError` enum
    - Add `ConvertErrorContext` struct
    - Add `ConvertErrorContextSource` enum
    - Add `ConvertResult` alias type
- `DisplayMode` enum add `check_image_tpye` function

### Refactored

- All convert functions will be returned as a `ConvertResult`

### Fixed

- Fix the convert logic problem in `Kitty` protocol when the given image is too small
- Fix all macros cannot find `Protocol::Auto` when `sixel` feature is disabled

## [v0.1.20] - 2025-10-31

### Fixed

- Fix the convert logic problem in the center mode

## [v0.1.19] - 2025-10-31

### Added

- Add `show_image!` macro to display image
- Add `show_images!` macro to display multiple images
- Add `process_images!` macro to process images

## [v0.1.18] - 2025-10-30

### Added

- Add the chaining operator to the `ImageProcessorOption` enum to make it easier to use
- Add `Protocol` enum.
- Add `DisplayModeBuilder` struct.
- Support automatic select protocol.
- Add relevant tests and examples.