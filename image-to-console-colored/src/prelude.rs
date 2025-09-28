use crate::shaders::Text;

pub trait ToColoredText {
    fn to_colored_text(&self) -> Text;
}

impl ToColoredText for str {
    fn to_colored_text(&self) -> Text {
        Text::new(String::from(self))
    }
}

impl ToColoredText for String {
    fn to_colored_text(&self) -> Text {
        Text::new(self.clone())
    }
}
