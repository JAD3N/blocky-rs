use thiserror::Error;
use crate::{AsJson, FromJson};
use super::Style;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("failed to parse json")]
    Parse,
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

pub trait ComponentContent {
    fn contents(&self) -> &str { "" }
}

pub trait Component: AsJson + ComponentContent + ComponentClone {
    fn siblings(&self) -> &Vec<Box<dyn Component>> where Self: Sized;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>> where Self: Sized;

    fn append(&mut self, component: Box<dyn Component>) where Self: Sized {
        self.siblings_mut().push(component);
    }

    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn set_style(&mut self, style: Style) {
        *self.style_mut() = style;
    }

    fn get_base_json(&self) -> serde_json::Value where Self: Sized {
        let mut value = json!({});

        // add style if not empty
        if !self.style().is_empty() {
            value["style"] = self.style().as_json();
        }

        // add siblings
        if !self.siblings().is_empty() {
            value["extra"] = self.siblings()
                .iter()
                .map(|sibling| sibling.as_json())
                .collect::<serde_json::Value>()
                .into();
        }

        value
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

impl FromJson for Box<dyn Component> {
    type Err = ComponentError;

    fn from_json(_value: &serde_json::Value) -> Result<Self, Self::Err> {
        Err(ComponentError::Parse)
    }
}

#[macro_export]
macro_rules! component {
    ($svis:vis struct $name:ident { $($fvis:vis $fname:ident: $fty:ty),* $(,)? }) => {
        #[derive(Clone)]
        $svis struct $name {
            siblings: Vec<Box<dyn Component>>,
            style: Style,
            $($fvis $fname: $fty),*
        }

        impl $crate::chat::Component for $name {
            fn siblings(&self) -> &Vec<Box<dyn Component>> {
                self.siblings.as_ref()
            }

            fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
                self.siblings.as_mut()
            }

            fn style(&self) -> &Style {
                &self.style
            }

            fn style_mut(&mut self) -> &mut Style {
                &mut self.style
            }
        }
    };
}
