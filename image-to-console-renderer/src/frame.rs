#[cfg(feature = "gif_player")]
#[derive(Clone)]
pub struct Frame {
    pub index: usize,
    pub frame: String,
    pub delay: u64,
}
#[cfg(feature = "gif_player")]
impl Frame {
    pub fn unpacking(&self) -> (&str, usize, u64) {
        (&self.frame, self.index, self.delay)
    }
}