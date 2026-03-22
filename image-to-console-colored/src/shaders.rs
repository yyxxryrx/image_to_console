use crate::colors::TerminalColor;
use std::fmt::{Display, Formatter};
use crate::styles::Styles;

pub struct TextHeader {
    foreground_color: Option<String>,
    background_color: Option<String>,
    style: Option<String>,
}

impl TextHeader {
    pub fn new(
        foreground_color: Option<String>,
        background_color: Option<String>,
        style: Option<String>,
    ) -> TextHeader {
        Self {
            foreground_color,
            background_color,
            style,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(None, None, None)
    }
}

impl Display for TextHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref fgc) = self.foreground_color {
            write!(f, "{fgc}")?;
        }
        if let Some(ref bgc) = self.background_color {
            write!(f, "{bgc}")?;
        }
        if let Some(ref sty) = self.style {
            write!(f, "{sty}")?;
        }
        Ok(())
    }
}

pub struct Text {
    pub content: String,
    head: TextHeader,
}

#[allow(dead_code)]
impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content,
            head: TextHeader::new_empty(),
        }
    }

    pub fn set_foreground_color(&mut self, foreground_color: TerminalColor) -> &mut Self {
        self.set_foreground_color_with_code(foreground_color as u8)
    }

    pub fn set_foreground_color_with_code(&mut self, foreground_color: u8) -> &mut Self {
        self.head.foreground_color = Some(format!("\x1b[38;5;{}m", foreground_color));
        self
    }

    pub fn set_foreground_color_rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.head.foreground_color = Some(format!("\x1b[38;2;{};{};{}m", r, g, b));
        self
    }

    pub fn set_background_color(&mut self, background_color: TerminalColor) -> &mut Self {
        self.set_background_color_with_code(background_color as u8)
    }

    pub fn set_background_color_with_code(&mut self, background_color: u8) -> &mut Self {
        self.head.background_color = Some(format!("\x1b[48;5;{}m", background_color));
        self
    }

    pub fn set_background_color_rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.head.background_color = Some(format!("\x1b[48;2;{};{};{}m", r, g, b));
        self
    }
    
    pub fn set_style(&mut self, style: Styles) -> &mut Self {
        self.head.style = Some(style.to_string());
        self
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}\x1b[0m", self.head.to_string(), self.content)
    }
}
