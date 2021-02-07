use std::fmt;
use blocky::ChatFormatting;

#[derive(Debug, Clone, PartialEq)]
pub struct TextColor {
    value: u32,
    name: Option<String>,
}

impl TextColor {
    pub fn new(value: u32, name: Option<String>) -> Self {
        Self { value, name }
    }

    pub fn new_with_empty_name(value: u32) -> Self {
        Self::new(value, None)
    }

    pub fn format_value(&self) -> String {
        format!("#{:06X}", self.value)
    }
}

impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name.as_ref() {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "{}", self.format_value()),
        }
    }
}

impl Into<TextColor> for ChatFormatting {
    fn into(self) -> TextColor {
        if self.is_color() {
            TextColor::new(
                self.color().unwrap(),
                Some(String::from(self.name()))
            )
        } else {
            panic!("only colors can be converted")
        }
    }
}
