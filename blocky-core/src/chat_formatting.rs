use std::fmt;
use std::collections::HashMap;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct ChatFormatting {
    name: &'static str,
    code: char,
    is_format: bool,
    id: i32,
    color: Option<u32>,
}

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new("[^a-z]").unwrap();
    static ref FORMATTING_BY_NAME: HashMap<String, &'static ChatFormatting> = {
        let mut m = HashMap::new();
        // iterate values and add into lookup
        for &value in ChatFormatting::VALUES.iter() {
            let name = ChatFormatting::clean_name(value.name());
            m.insert(name, value);
        }
        m
    };
}

impl ChatFormatting {
    pub const BLACK: Self = Self::new_color("black", '0', 0, Some(0));
    pub const DARK_BLUE: Self = Self::new_color("dark_blue", '1', 1, Some(170));
    pub const DARK_GREEN: Self = Self::new_color("dark_green", '2', 2, Some(43520));
    pub const DARK_AQUA: Self = Self::new_color("dark_aqua", '3', 3, Some(43690));
    pub const DARK_RED: Self = Self::new_color("dark_red", '4', 4, Some(11141120));
    pub const DARK_PURPLE: Self = Self::new_color("dark_purple", '5', 5, Some(11141290));
    pub const GOLD: Self = Self::new_color("gold", '6', 6, Some(16755200));
    pub const GRAY: Self = Self::new_color("gray", '7', 7, Some(11184810));
    pub const DARK_GRAY: Self = Self::new_color("dark_gray", '8', 8, Some(5592405));
    pub const BLUE: Self = Self::new_color("blue", '9', 9, Some(5592575));
    pub const GREEN: Self = Self::new_color("green", 'a', 10, Some(5635925));
    pub const AQUA: Self = Self::new_color("aqua", 'b', 11, Some(5636095));
    pub const RED: Self = Self::new_color("red", 'c', 12, Some(16733525));
    pub const LIGHT_PURPLE: Self = Self::new_color("light_purple", 'd', 13, Some(16733695));
    pub const YELLOW: Self = Self::new_color("yellow", 'e', 14, Some(16777045));
    pub const WHITE: Self = Self::new_color("white", 'f', 15, Some(16777215));

    pub const OBFUSCATED: Self = Self::new_format("obfuscated", 'k');
    pub const BOLD: Self = Self::new_format("bold", 'l');
    pub const STRIKETHROUGH: Self = Self::new_format("strikethrough", 'm');
    pub const UNDERLINE: Self = Self::new_format("underline", 'n');
    pub const ITALIC: Self = Self::new_format("italic", 'o');

    pub const RESET: Self = Self::new_color("reset", 'r', -1, None);

    pub const VALUES: [&'static Self; 21] = [
        &Self::BLACK,
        &Self::DARK_BLUE,
        &Self::DARK_GREEN,
        &Self::DARK_AQUA,
        &Self::DARK_RED,
        &Self::DARK_PURPLE,
        &Self::GOLD,
        &Self::GRAY,
        &Self::DARK_GRAY,
        &Self::BLUE,
        &Self::GREEN,
        &Self::AQUA,
        &Self::RED,
        &Self::LIGHT_PURPLE,
        &Self::YELLOW,
        &Self::WHITE,

        &Self::OBFUSCATED,
        &Self::STRIKETHROUGH,
        &Self::UNDERLINE,
        &Self::ITALIC,

        &Self::RESET,
    ];

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

    pub fn clean_name(name: &str) -> String {
        let name = name.to_lowercase();

        // remove non a-z to make look ups easier
        NAME_REGEX.replace_all(&name, "")
            .to_string()
    }

    pub fn get_by_name(s: &str) -> Option<&'static ChatFormatting> {
        FORMATTING_BY_NAME.get(&Self::clean_name(s))
            .and_then(|value| Some(*value))
    }
}

impl fmt::Display for ChatFormatting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "§{}", self.code)
    }
}
