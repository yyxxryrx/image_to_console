use std::path::Path;

use image_to_console_core::DisplayMode as CoreDisplayMode;
use pyo3::{
    exceptions::{PyFileNotFoundError, PyOSError},
    prelude::*,
};

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    HalfColor,
    FullColor,
    FullNoColor,
    Ascii,
    WezTerm,
    WezTermNoColor,
    Kitty,
    KittyNoColor,
    Iterm2,
    Iterm2NoColor,
    SixelHalf,
    SixelFull,
}

impl From<DisplayMode> for CoreDisplayMode {
    fn from(value: DisplayMode) -> Self {
        match value {
            DisplayMode::HalfColor => CoreDisplayMode::HalfColor,
            DisplayMode::FullColor => CoreDisplayMode::FullColor,
            DisplayMode::FullNoColor => CoreDisplayMode::FullNoColor,
            DisplayMode::Ascii => CoreDisplayMode::Ascii,
            DisplayMode::WezTerm => CoreDisplayMode::WezTerm,
            DisplayMode::WezTermNoColor => CoreDisplayMode::WezTermNoColor,
            DisplayMode::Kitty => CoreDisplayMode::Kitty,
            DisplayMode::KittyNoColor => CoreDisplayMode::KittyNoColor,
            DisplayMode::Iterm2 => CoreDisplayMode::Iterm2,
            DisplayMode::Iterm2NoColor => CoreDisplayMode::Iterm2NoColor,
            DisplayMode::SixelHalf => CoreDisplayMode::SixelHalf,
            DisplayMode::SixelFull => CoreDisplayMode::SixelFull,
        }
    }
}

#[pyclass]
struct Image {
    img: image::DynamicImage,
}

#[pyclass]
struct DisplayImage {
    result: image_to_console_core::processor::ImageProcessorResult,
}

#[pymethods]
impl Image {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        match image::load_from_memory(data) {
            Ok(img) => Ok(Self { img }),
            Err(e) => Err(PyOSError::new_err(e.to_string())),
        }
    }

    #[pyo3(signature = (
        mode = DisplayMode::FullColor,
        center = false,
    ))]
    pub fn display(&self, mode: DisplayMode, center: bool) -> DisplayImage {
        let mode = CoreDisplayMode::from(mode);
        let option = image_to_console_core::processor::ImageProcessorOptions::new(
            mode,
            if mode.is_normal() || mode.is_sixel() {
                image_to_console_core::ResizeMode::default()
            } else {
                image_to_console_core::ResizeMode::None
            },
            center,
        );
        let result =
            image_to_console_core::processor::ImageProcessor::new(self.img.clone(), option)
                .process();
        DisplayImage { result }
    }
}

#[pymethods]
impl DisplayImage {
    fn __str__(&self) -> String {
        self.result.display().to_string()
    }
}

#[pyfunction]
fn open(path: String) -> PyResult<Image> {
    let path = Path::new(&path);
    if !path.is_file() {
        return Err(PyFileNotFoundError::new_err("cannot found file"));
    }
    let img = image::open(path).map_err(|e| PyOSError::new_err(e.to_string()))?;
    Ok(Image { img })
}

#[pymodule]
fn image_to_console_core_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    m.add_class::<DisplayMode>()?;
    m.add_class::<DisplayImage>()?;
    m.add_function(wrap_pyfunction!(open, m)?)?;
    Ok(())
}
