use std::fmt;

#[derive(PartialEq)]
pub struct ChatFormatting {
    name: &'static str,
    code: char,
    is_format: bool,
    id: i32,
    color: Option<u32>,
}

impl ChatFormatting {
    pub const BLACK: ChatFormatting = Self::new_color("BLACK", '0', 0, Some(0));
    pub const DARK_BLUE: ChatFormatting = Self::new_color("DARK_BLUE", '1', 1, Some(170));
    pub const DARK_GREEN: ChatFormatting = Self::new_color("DARK_GREEN", '2', 2, Some(43520));
    pub const DARK_AQUA: ChatFormatting = Self::new_color("DARK_AQUA", '3', 3, Some(43690));
    pub const DARK_RED: ChatFormatting = Self::new_color("DARK_RED", '4', 4, Some(11141120));
    pub const DARK_PURPLE: ChatFormatting = Self::new_color("DARK_PURPLE", '5', 5, Some(11141290));

    pub const GOLD: ChatFormatting = Self::new_color("GOLD", '6', 6, Some(16755200));
    pub const GRAY: ChatFormatting = Self::new_color("GRAY", '7', 7, Some(11184810));
    pub const DARK_GRAY: ChatFormatting = Self::new_color("DARK_GRAY", '8', 8, Some(5592405));
    pub const BLUE: ChatFormatting = Self::new_color("BLUE", '9', 9, Some(5592575));
    pub const GREEN: ChatFormatting = Self::new_color("GREEN", 'a', 10, Some(5635925));
    pub const AQUA: ChatFormatting = Self::new_color("AQUA", 'b', 11, Some(5636095));
    pub const RED: ChatFormatting = Self::new_color("RED", 'c', 12, Some(16733525));
    pub const LIGHT_PURPLE: ChatFormatting = Self::new_color("LIGHT_PURPLE", 'd', 13, Some(16733695));
    pub const YELLOW: ChatFormatting = Self::new_color("YELLOW", 'e', 14, Some(16777045));
    pub const WHITE: ChatFormatting = Self::new_color("WHITE", 'f', 15, Some(16777215));

    pub const OBFUSCATED: ChatFormatting = Self::new_format("OBFUSCATED", 'k');
    pub const BOLD: ChatFormatting = Self::new_format("BOLD", 'l');
    pub const STRIKETHROUGH: ChatFormatting = Self::new_format("STRIKETHROUGH", 'm');
    pub const UNDERLINE: ChatFormatting = Self::new_format("UNDERLINE", 'n');
    pub const ITALIC: ChatFormatting = Self::new_format("ITALIC", 'o');

    pub const RESET: ChatFormatting = Self::new_color("RESET", 'r', -1, None);

    const fn new(
        name: &'static str,
        code: char,
        is_format: bool,
        id: i32,
        color: Option<u32>,
    ) -> Self {
        Self {
            name,
            code,
            is_format,
            id,
            color,
        }
    }

    const fn new_color(
        name: &'static str,
        code: char,
        id: i32,
        color: Option<u32>,
    ) -> Self {
        Self::new(name, code, false, id, color)
    }

    const fn new_format(
        name: &'static str,
        code: char,
    ) -> Self {
        Self::new(name, code, true, -1, None)
    }

    pub fn is_color(&self) -> bool {
        !self.is_format && *self != Self::RESET
    }

    pub fn is_format(&self) -> bool {
        self.is_format
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> char {
        self.code
    }

    pub fn color(&self) -> Option<u32> {
        self.color
    }
}

impl fmt::Display for ChatFormatting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "§{}", self.code)
    }
}
