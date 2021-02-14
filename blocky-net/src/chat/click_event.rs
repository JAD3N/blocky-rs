use std::str::FromStr;
use thiserror::Error;
use crate::{AsJson, FromJson};

#[derive(Error, Debug)]
pub enum ClickActionError {
    #[error("invalid click action: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClickAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

impl ClickAction {
    pub fn name(&self) -> &str {
        match self {
            Self::OpenUrl => "open_url",
            Self::OpenFile => "open_file",
            Self::RunCommand => "run_command",
            Self::SuggestCommand => "suggest_command",
            Self::ChangePage => "change_page",
            Self::CopyToClipboard => "copy_to_clipboard",
        }
    }

    pub fn is_allowed_from_server(&self) -> bool {
        match self {
            Self::OpenFile => false,
           _ => true,
        }
    }
}

impl FromStr for ClickAction {
    type Err = ClickActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open_url" => Ok(Self::OpenUrl),
            "open_file" => Ok(Self::OpenFile),
            "run_command" => Ok(Self::RunCommand),
            "suggest_command" => Ok(Self::SuggestCommand),
            "change_page" => Ok(Self::ChangePage),
            "copy_to_clipboard" => Ok(Self::CopyToClipboard),
            _ => Err(ClickActionError::Invalid(s.into())),
        }
    }
}

#[derive(Error, Debug)]
pub enum ClickEventError {
    #[error("failed to parse json into click event")]
    Parse,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClickEvent {
    action: ClickAction,
    value: String,
}

impl ClickEvent {
    pub fn new<S: Into<String>>(action: ClickAction, value: S) -> Self {
        let value = value.into();
        Self { action, value }
    }

    pub fn action(&self) -> &ClickAction {
        &self.action
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl AsJson for ClickEvent {
    fn as_json(&self) -> serde_json::Value {
        json!({
            "action": self.action.name(),
            "value": self.value,
        })
    }
}

impl FromJson for ClickEvent {
    type Err = ClickEventError;

    fn from_json(value: &serde_json::Value) -> Result<Self, Self::Err> {
        if value.is_object() {
            let map = value.as_object().unwrap();

            if map.contains_key("action") && map.contains_key("value") {
                let action = map.get("action").unwrap();
                let value = map.get("value").unwrap();

                if action.is_string() && value.is_string() {
                    let action = action.as_str().unwrap();
                    let value = value.as_str()
                        .unwrap()
                        .into();

                    if let Ok(action) = ClickAction::from_str(action) {
                        return Ok(Self {
                            action,
                            value,
                        });
                    }
                }
            }
        }

        Err(ClickEventError::Parse)
    }
}
