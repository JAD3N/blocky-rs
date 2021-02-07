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

#[derive(Debug, Clone, PartialEq)]
pub struct ClickEvent {
    action: ClickAction,
    value: String,
}

impl ClickEvent {
    pub fn action(&self) -> &ClickAction {
        &self.action
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}