use crate::AsJson;
use super::{
    Component,
    ComponentClone,
    ComponentContent,
    Style,
};

#[derive(Clone)]
pub enum TranslatableComponentArg {
    String(String),
    Component(Box<dyn Component>),
}

impl AsJson for TranslatableComponentArg {
    fn as_json(&self) -> serde_json::Value {
        match self {
            Self::String(s) => s.clone().into(),
            Self::Component(c) => c.as_json(),
        }
    }
}

component!(pub struct TranslatableComponent {
    key: String,
    args: Vec<TranslatableComponentArg>,
});

impl TranslatableComponent {
    pub fn new<S: Into<String>>(key: S, args: Vec<TranslatableComponentArg>) -> Self {
        Self {
            siblings: vec![],
            style: Style::default(),
            key: key.into(),
            args,
        }
    }

    pub fn new_with_empty_args<S: Into<String>>(key: S) -> Self {
        Self::new(key, vec![])
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn key_mut(&mut self) -> &mut String {
        &mut self.key
    }

    pub fn set_key<S: Into<String>>(&mut self, key: S) {
        self.key = key.into();
    }

    pub fn args(&self) -> &Vec<TranslatableComponentArg> {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Vec<TranslatableComponentArg> {
        &mut self.args
    }

    pub fn set_args(&mut self, args: Vec<TranslatableComponentArg>) {
        self.args = args;
    }
}

impl AsJson for TranslatableComponent {
    fn as_json(&self) -> serde_json::Value {
        let mut value = self.get_base_json();
        value["translate"] = self.key.clone().into();
        value["with"] = self.args
            .iter()
            .map(|arg| arg.as_json())
            .collect::<serde_json::Value>()
            .into();
        value
    }
}

impl ComponentContent for TranslatableComponent {}

impl ComponentClone for TranslatableComponent {
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(Self {
            siblings: vec![],
            style: Style::default(),
            key: self.key.clone(),
            args: self.args.clone(),
        })
    }
}
