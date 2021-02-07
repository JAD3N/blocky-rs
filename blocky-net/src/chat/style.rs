use super::{TextColor, ClickEvent};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    color: Option<TextColor>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    click_event: Option<ClickEvent>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            click_event: None,
        }
    }
}