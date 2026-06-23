pub const IMAGE_EXTS: [&str; 6] = ["png", "jpg", "jpeg", "webp", "tiff", "bmp"];

#[cfg(target_os = "linux")]
pub const DEFAULT_LEN: std::num::NonZeroUsize = match std::num::NonZeroUsize::new(200) {
    Some(v) => v,
    None => unreachable!(),
};
