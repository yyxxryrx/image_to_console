pub struct Config {
    pub clear: bool,
    pub pause: bool,
    pub center: bool,
    pub show_time: bool,
    pub fps: Option<u64>,
    pub disable_info: bool,
    pub disable_print: bool,
    pub show_file_name: bool,
    pub output: Option<String>,
    pub file_name: Option<String>,
    #[cfg(feature = "rodio")]
    pub audio: crate::audio_path::AudioPath,
    #[cfg(feature = "sixel_support")]
    pub mode: image_to_console_core::DisplayMode,
}