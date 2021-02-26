use super::{Style, TextComponent, TranslatableComponent, TranslatableComponentArg};
use crate::{AsJson, FromJson};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("failed to parse json")]
    Parse,
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

pub trait ComponentContent {
    fn contents(&self) -> &str {
        ""
    }
}

pub trait Component: mopa::Any + AsJson + ComponentContent + ComponentClone {
    fn siblings(&self) -> &Vec<Box<dyn Component>>;
    fn siblings_mut(&mut self) -> &mut Vec<Box<dyn Component>>;

    fn append(&mut self, component: Box<dyn Component>) {
        self.siblings_mut().push(component);
    }

    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn set_style(&mut self, style: Style) {
        *self.style_mut() = style;
    }

    fn get_base_json(&self) -> serde_json::Value {
        let mut value = json!({});

        // convert style to json
        let style = self.style().as_json();

        // copy style attributes into value
        if let serde_json::Value::Object(m) = style {
            for entry in m {
                value[entry.0] = entry.1;
            }
        }

        // add siblings
        if !self.siblings().is_empty() {
            value["extra"] = self
                .siblings()
                .iter()
                .map(|sibling| sibling.as_json())
                .collect::<serde_json::Value>()
                .into();
        }

        value
    }
}

mopafy!(Component);

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

impl FromJson for Box<dyn Component> {
    type Err = ComponentError;

    fn from_json(value: &serde_json::Value) -> Result<Self, Self::Err> {
        if value.is_string() {
            Ok(Box::new(TextComponent::new(value.as_str().unwrap())))
        } else if value.is_object() {
            let obj = value.as_object().unwrap();
            let mut c: Box<dyn Component> = if let Some(text) = obj.get("text") {
                // ensure text is a string
                let text = text.as_str().ok_or(ComponentError::Parse)?;
                Box::new(TextComponent::new(text))
            } else if let Some(translate) = obj.get("translate") {
                // ensure translate is a string
                let translate = translate.as_str().ok_or(ComponentError::Parse)?;
                let mut args = vec![];

                if let Some(with) = obj.get("with") {
                    // ensure with is an array
                    let with = with.as_array().ok_or(ComponentError::Parse)?;

                    // iterate args
                    for arg in with {
                        if let Ok(sub_c) = Self::from_json(arg) {
                            if sub_c.is::<TextComponent>()
                                && sub_c.style().is_empty()
                                && sub_c.siblings().is_empty()
                            {
                                let text = sub_c.contents().into();
                                args.push(TranslatableComponentArg::String(text));
                            } else {
                                args.push(TranslatableComponentArg::Component(sub_c));
                            }
                        }
                    }
                }

                Box::new(TranslatableComponent::new(translate, args))
            } else {
                return Err(ComponentError::Parse);
            };

            if obj.contains_key("extra") && obj["extra"].is_array() {
                for entry in value["extra"].as_array().unwrap() {
                    c.append(Self::from_json(entry)?);
                }
            }

            // try parse style
            if let Ok(style) = Style::from_json(value) {
                if !style.is_empty() {
                    c.set_style(style);
                }
            }

            Ok(c)
        } else {
            Err(ComponentError::Parse)
        }
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
