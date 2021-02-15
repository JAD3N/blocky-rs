use crate::AsJson;
use super::{
    Component,
    ComponentClone,
    ComponentContent,
    Style,
};

component!(pub struct TextComponent {
    text: String,
});

impl TextComponent {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            siblings: vec![],
            style: Style::default(),
            text: text.into(),
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }
}

impl AsJson for TextComponent {
    fn as_json(&self) -> serde_json::Value {
        let mut value = self.get_base_json();
        value["text"] = self.text.clone().into();
        value
    }
}

impl ComponentContent for TextComponent {
    fn contents(&self) -> &str {
        &self.text
    }
}

impl ComponentClone for TextComponent {
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(Self {
            siblings: vec![],
            style: Style::default(),
            text: self.text.clone(),
        })
    }
}
