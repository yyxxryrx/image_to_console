use crate::colors::TerminalColor;

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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(fgc) = self.foreground_color.clone() {
            result.push_str(&fgc);
        }
        if let Some(bgc) = self.background_color.clone() {
            result.push_str(&bgc);
        }
        if let Some(style) = self.style.clone() {
            result.push_str(&style);
        }
        result
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

    pub fn to_string(&self) -> String {
        format!("{}{}\x1b[0m", self.head.to_string(), self.content)
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
