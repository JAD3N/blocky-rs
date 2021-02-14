use crate::AsJson;
use super::{TextColor, ClickEvent};
use blocky_core::ResourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub color: Option<TextColor>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub click_event: Option<ClickEvent>,
    // TODO: pub hover_event: Option<HoverEvent>,
    pub insertion: Option<String>,
    pub font: Option<ResourceLocation>,
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
            // hover_event: None,
            insertion: None,
            font: None,
        }
    }
}

impl Style {
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.click_event.is_none()
            // && self.hover_event.is_none()
            && self.insertion.is_none()
            && self.font.is_none()
    }
}

impl AsJson for Style {
    fn as_json(&self) -> serde_json::Value {
        if self.is_empty() {
            serde_json::Value::Null
        } else {
            let mut obj = json!({});

            // only set color if non-null
            if let Some(color) = &self.color {
                obj["color"] = color.to_string().into();
            }

            // only set bold if non-null
            if let Some(bold) = self.bold {
                obj["bold"] = bold.into();
            }

            // only set italic if non-null
            if let Some(italic) = self.italic {
                obj["italic"] = italic.into();
            }

            // only set underlined if non-null
            if let Some(underlined) = self.underlined {
                obj["underlined"] = underlined.into();
            }

            // only set strikethrough if non-null
            if let Some(strikethrough) = self.strikethrough {
                obj["strikethrough"] = strikethrough.into();
            }

            // only set obfuscated if non-null
            if let Some(obfuscated) = self.obfuscated {
                obj["obfuscated"] = obfuscated.into();
            }

            if let Some(click_event) = &self.click_event {
                obj["clickEvent"] = click_event.as_json();
            }

            if let Some(insertion) = &self.insertion {
                obj["insertion"] = insertion.clone().into();
            }

            if let Some(font) = &self.font {
                obj["font"] = font.to_string().into();
            }

            obj
        }
    }
}
