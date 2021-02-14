use super::{Component, ComponentClone, ComponentContent, Style};

component!(pub struct TextComponent {
    text: String,
});

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
