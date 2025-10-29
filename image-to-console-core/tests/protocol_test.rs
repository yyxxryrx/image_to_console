use image_to_console_core::{
    DisplayMode,
    protocol::{DisplayModeBuilder, Protocol},
};

#[test]
fn test_protocol() {
    // Tests Table
    // |Protocol        |IsFull|HasColor|DisplayMode              |
    let tests = vec![
        (Protocol::Normal, true, true, DisplayMode::FullColor),
        (Protocol::Normal, true, false, DisplayMode::FullNoColor),
        (Protocol::Normal, false, true, DisplayMode::HalfColor),
        (Protocol::Normal, false, false, DisplayMode::Ascii),
        (Protocol::Kitty, true, true, DisplayMode::Kitty),
        (Protocol::Kitty, true, false, DisplayMode::KittyNoColor),
        (Protocol::ITerm2, true, true, DisplayMode::Iterm2),
        (Protocol::ITerm2, true, false, DisplayMode::Iterm2NoColor),
        (Protocol::WezTerm, true, true, DisplayMode::WezTerm),
        (Protocol::WezTerm, true, false, DisplayMode::WezTermNoColor),
        #[cfg(feature = "sixel")]
        (Protocol::Sixel, true, true, DisplayMode::SixelFull),
        #[cfg(feature = "sixel")]
        (Protocol::Sixel, false, true, DisplayMode::SixelHalf),
    ];
    for (protocol, is_full, has_color, display_mode) in tests {
        let mode = DisplayModeBuilder::new(protocol)
            .option_is_full(is_full)
            .option_has_color(has_color)
            .build();
        assert_eq!(mode, display_mode);
    }
}
