use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Styles {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    ReverseDisplay,
    Invis,
    DeleteLine,
}

impl Display for Styles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Bold => "\x1b[1m",
            Self::Dim => "\x1b[2m",
            Self::Italic => "\x1b[3m",
            Self::Underline => "\x1b[4m",
            Self::Blink => "\x1b[5m",
            Self::ReverseDisplay => "\x1b[6m",
            Self::Invis => "\x1b[7m",
            Self::DeleteLine => "\x1b[8m"
        };
        write!(f, "{code}")
    }
}