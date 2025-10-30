#[derive(Debug, Clone)]
pub enum AudioPath {
    #[cfg(feature = "rodio")]
    Temp(std::path::PathBuf),
    #[cfg(feature = "rodio")]
    Custom(std::path::PathBuf),
    None,
}

impl Default for AudioPath {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(feature = "rodio")]
impl Drop for AudioPath {
    fn drop(&mut self) {
        if let AudioPath::Temp(path) = self {
            std::fs::remove_file(path).unwrap();
        }
    }
}

impl AudioPath {
    #[cfg(feature = "rodio")]
    pub fn get_path(&self) -> Option<std::path::PathBuf> {
        match self {
            AudioPath::Temp(path) => Some(path.clone()),
            AudioPath::Custom(path) => Some(path.clone()),
            AudioPath::None => None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, AudioPath::None)
    }
}
