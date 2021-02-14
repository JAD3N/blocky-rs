use std::fmt;
use std::collections::HashMap;
use blocky_core::ChatFormatting;

#[derive(Debug, Clone, PartialEq)]
pub struct TextColor {
    value: u32,
    name: Option<String>,
}

lazy_static! {
    static ref NAMED_COLORS: HashMap<String, TextColor> = {
        let mut m = HashMap::new();

        for &format in &ChatFormatting::VALUES {
            if format.is_color() {
                m.insert(format.name().to_string(), format.into());
            }
        }

        m
    };
}

impl TextColor {
    pub fn new(value: u32, name: Option<String>) -> Self {
        Self { value, name }
    }

    pub fn from_color(value: u32) -> Self {
        Self::new(value, None)
    }

    pub fn format_value(&self) -> String {
        format!("#{:06X}", self.value)
    }

    pub fn parse(s: &str) -> Option<TextColor> {
        if s.starts_with("#") {
            // try hex parse string into u32
            let color = u32::from_str_radix(&s[1..], 16).ok()?;
            Some(TextColor::from_color(color))
        } else {
            // clone named color if found
            NAMED_COLORS.get(s).and_then(|color| Some(color.clone()))
        }
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

impl Into<TextColor> for &ChatFormatting {
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
