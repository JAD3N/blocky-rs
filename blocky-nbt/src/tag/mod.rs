mod kind;
mod index;
mod parser;

pub use kind::*;
pub use index::*;
pub use parser::*;

#[cfg(feature = "preserve-order")]
pub use indexmap::IndexMap as Map;
#[cfg(not(feature = "preserve-order"))]
pub use std::collections::HashMap as Map;

use regex::Regex;
use std::fmt;

lazy_static! {
    static ref SIMPLE_PATTERN: Regex = Regex::new("^[A-Za-z0-9._+-]+$").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Self>),
    Compound(Map<String, Self>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

macro_rules! into_tag {
    ($input:ty, $output:ident) => {
        impl Into<Tag> for $input {
            fn into(self) -> Tag {
                Tag::$output(self)
            }
        }

        impl Into<$input> for Tag {
            fn into(self) -> $input {
                match self {
                    Tag::$output(value) => value,
                    _ => panic!("cannot convert tag"),
                }
            }
        }

        impl<'b> Into<$input> for &'b Tag {
            fn into(self) -> $input {
                match self {
                    Tag::$output(value) => value.to_owned(),
                    _ => panic!("cannot convert tag"),
                }
            }
        }
    };
}

macro_rules! into_tags {
    { $({ $input:ty, $output:ident }),* $(,)* } => {
        $(into_tag!($input, $output);)*
    };
}

into_tags! {
    { i8, Byte },
    { i16, Short },
    { i32, Int },
    { i64, Long },

    { f32, Float },
    { f64, Double },

    { String, String },

    { Vec<i8>, ByteArray },
    { Vec<i32>, IntArray },
    { Vec<i64>, LongArray },
}

impl Tag {
    pub fn kind(&self) -> Kind {
        match self {
            Self::End => Kind::End,
            Self::Byte(_) => Kind::Byte,
            Self::Short(_) => Kind::Short,
            Self::Int(_) => Kind::Int,
            Self::Long(_) => Kind::Long,
            Self::Float(_) => Kind::Float,
            Self::Double(_) => Kind::Double,
            Self::ByteArray(_) => Kind::ByteArray,
            Self::String(_) => Kind::String,
            Self::List(v) => Kind::List(
                v.get(0)
                    .map(|tag| tag.kind().id())
                    .unwrap_or(Kind::End.id()),
            ),
            Self::Compound(_) => Kind::Compound,
            Self::IntArray(_) => Kind::IntArray,
            Self::LongArray(_) => Kind::LongArray,
        }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Self> {
        index.index_into(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Self> {
        index.index_into_mut(self)
    }

    pub fn insert<I: Index>(&mut self, index: I, value: Self) {
        *index.index_or_insert(self) = value;
    }
}

fn quote_and_escape(s: &str) -> String {
    let mut builder = String::new();
    let mut quote_chr = None;

    for chr in s.chars() {
        if chr == '\\' {
            builder.push('\\');
        } else if chr == '"' || chr == '\'' {
            if quote_chr.is_none() {
                quote_chr = Some(if chr == '"' { '\'' } else { '"' });
            }

            if quote_chr.is_some() && quote_chr.unwrap() == chr {
                builder.push('\\');
            }
        }

        builder.push(chr);
    }

    let quote_chr = quote_chr.unwrap_or('"');

    builder.insert(0, quote_chr);
    builder.push(quote_chr);
    builder
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::End => panic!("cannot convert end to string"),

            Self::Byte(value) => format!("{}b", *value),
            Self::Short(value) => format!("{}s", *value),
            Self::Int(value) => format!("{}", *value),
            Self::Long(value) => format!("{}L", *value),

            Self::Float(value) => format!("{}f", *value),
            Self::Double(value) => format!("{}d", *value),

            Self::String(s) => quote_and_escape(&s),

            Self::List(v) => {
                let mut items = vec![];

                for tag in v {
                    items.push(tag.to_string());
                }

                format!("[{}]", items.join(","))
            },

            Self::Compound(m) => {
                let mut items = vec![];

                for (name, tag) in m {
                    let value = tag.to_string();
                    let key = if SIMPLE_PATTERN.is_match(&name) {
                        name.clone()
                    } else {
                        quote_and_escape(&name)
                    };

                    items.push(format!("{}:{}", key, value));
                }

                format!("{{{}}}", items.join(","))
            },

            Self::ByteArray(v) => {
                let mut items = vec![];

                for value in v {
                    items.push(Tag::Byte(*value).to_string());
                }

                format!("[B;{}]", items.join(","))
            },
            Self::IntArray(v) => {
                let mut items = vec![];

                for value in v {
                    items.push(Tag::Int(*value).to_string());
                }

                format!("[I;{}]", items.join(","))
            },
            Self::LongArray(v) => {
                let mut items = vec![];

                for value in v {
                    items.push(Tag::Long(*value).to_string());
                }

                format!("[L;{}]", items.join(","))
            },
        })
    }
}

#[macro_export]
macro_rules! tag {
    ($s:expr) => {
        $crate::Nbt::parse(format!("{}", $s))
            .expect("failed to parse tag")
    };
}
