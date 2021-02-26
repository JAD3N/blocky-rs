use std::str::FromStr;
use blocky_core::ResourceLocation;
use thiserror::Error;
use crate::{AsJson, FromJson};
use super::{ClickEvent, TextColor};

#[derive(Error, Debug)]
pub enum StyleError {
    #[error("invalid type for: {0}")]
    InvalidType(String),
    #[error("invalid color: {0}")]
    InvalidColor(String),
    #[error("failed to parse style")]
    Parse,
}

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

impl FromJson for Style {
    type Err = StyleError;

    fn from_json(value: &serde_json::Value) -> Result<Self, Self::Err> {
        let mut style = Style::default();

        if let Some(color) = value.get("color") {
            let color = color
                .as_str()
                .ok_or(StyleError::InvalidType(String::from("color")))?;

            if let Some(color) = TextColor::parse(color) {
                style.color = Some(color);
            } else {
                return Err(StyleError::InvalidColor(color.into()));
            }
        }

        if let Some(bold) = value.get("bold") {
            let bold = bold
                .as_bool()
                .ok_or(StyleError::InvalidType(String::from("bold")))?;

            style.bold = Some(bold);
        }

        if let Some(italic) = value.get("italic") {
            let italic = italic
                .as_bool()
                .ok_or(StyleError::InvalidType(String::from("italic")))?;

            style.italic = Some(italic);
        }

        if let Some(underlined) = value.get("underlined") {
            let underlined = underlined
                .as_bool()
                .ok_or(StyleError::InvalidType(String::from("underlined")))?;

            style.underlined = Some(underlined);
        }

        if let Some(strikethrough) = value.get("strikethrough") {
            let strikethrough = strikethrough
                .as_bool()
                .ok_or(StyleError::InvalidType(String::from("strikethrough")))?;

            style.strikethrough = Some(strikethrough);
        }

        if let Some(obfuscated) = value.get("obfuscated") {
            let obfuscated = obfuscated
                .as_bool()
                .ok_or(StyleError::InvalidType(String::from("obfuscated")))?;

            style.obfuscated = Some(obfuscated);
        }

        if let Some(click_event) = value.get("clickEvent") {
            let click_event = ClickEvent::from_json(click_event)
                .or(Err(StyleError::Parse))?;

            style.click_event = Some(click_event);
        }

        if let Some(insertion) = value.get("insertion") {
            let insertion = insertion
                .as_str()
                .ok_or(StyleError::InvalidType(String::from("insertion")))?;

            style.insertion = Some(insertion.into());
        }

        if let Some(font) = value.get("font") {
            let font = font
                .as_str()
                .ok_or(StyleError::InvalidType(String::from("font")))?;

            let loc = ResourceLocation::from_str(font)
                .or(Err(StyleError::Parse))?;

            style.font = Some(loc);
        }

        Ok(style)
    }
}
