mod reader;

use reader::*;

use crate::{Tag, Nbt, Kind, Map};
use regex::Regex;

lazy_static! {
    static ref DOUBLE_PATTERN_NOSUFFIX: Regex = Regex::new("^(?i)[-+]?(?:[0-9]+[.]|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?$").unwrap();
    static ref DOUBLE_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?d$").unwrap();
    static ref FLOAT_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:[0-9]+[.]?|[0-9]*[.][0-9]+)(?:e[-+]?[0-9]+)?f$").unwrap();
    static ref BYTE_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:0|[1-9][0-9]*)b$").unwrap();
    static ref LONG_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:0|[1-9][0-9]*)l$").unwrap();
    static ref SHORT_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:0|[1-9][0-9]*)s$").unwrap();
    static ref INT_PATTERN: Regex = Regex::new("^(?i)[-+]?(?:0|[1-9][0-9]*)$").unwrap();
}

macro_rules! read_array {
    ($self:ident, $inner:ident, $typ:ty) => {
        {
            let mut v: Vec<$typ> = vec![];

            while $self.reader.peek()? != ']' {
                let start = $self.reader.position();
                let tag = $self.read_value()?;

                if tag.kind() != Kind::$inner {
                    $self.reader.set_position(start);
                    anyhow::bail!("array has mixed tags");
                }

                v.push(tag.into());

                if !$self.has_separator()? {
                    break;
                }

                if $self.reader.done() {
                    anyhow::bail!("expected array closure")
                }
            }

            v.into()
        }
    };
}

pub struct Parser {
    reader: Reader,
}

impl Parser {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { reader: Reader::new(s.into()) }
    }

    fn expect(&mut self, chr: char) -> anyhow::Result<()> {
        self.reader.skip_whitespace()?;
        self.reader.expect(chr)?;

        Ok(())
    }

    fn read_key(&mut self) -> anyhow::Result<String> {
        self.reader.skip_whitespace()?;

        if self.reader.done() {
            anyhow::bail!("missing expected key");
        }

        self.reader.read_string()
    }

    fn read_typed_value(&mut self) -> anyhow::Result<Tag> {
        self.reader.skip_whitespace()?;

        let start = self.reader.position();

        if Reader::is_quote(self.reader.peek()?) {
            Ok(self.reader.read_quoted_string()?.into())
        } else {
            let s = self.reader.read_unquoted_string()?;

            if s.is_empty() {
                self.reader.set_position(start);
                Err(anyhow::anyhow!("missing expected value"))
            } else {
                Self::parse_type(&s).or_else(|_| Ok(Tag::String(s)))
            }
        }
    }

    fn parse_type(s: &str) -> anyhow::Result<Tag> {
        let len = s.len();

        Ok(if FLOAT_PATTERN.is_match(&s) {
            Tag::Float(s[..len - 1].parse()?)
        } else if BYTE_PATTERN.is_match(&s) {
            Tag::Byte(s[..len - 1].parse()?)
        } else if LONG_PATTERN.is_match(&s) {
            Tag::Long(s[..len - 1].parse()?)
        } else if SHORT_PATTERN.is_match(&s) {
            Tag::Short(s[..len - 1].parse()?)
        } else if INT_PATTERN.is_match(&s) {
            Tag::Int(s.parse()?)
        } else if DOUBLE_PATTERN.is_match(&s) {
            Tag::Double(s[..len - 1].parse()?)
        } else if DOUBLE_PATTERN_NOSUFFIX.is_match(&s) {
            Tag::Double(s.parse()?)
        } else if s.to_lowercase() == "true" {
            Tag::Byte(1)
        } else if s.to_lowercase() == "false" {
            Tag::Byte(0)
        } else {
            anyhow::bail!("unknown type")
        })
    }

    pub fn read_value(&mut self) -> anyhow::Result<Tag> {
        self.reader.skip_whitespace()?;

        if self.reader.done() {
            anyhow::bail!("missing expected value");
        }

        let chr = self.reader.peek()?;

        if chr == '{' {
            self.read_struct()
        } else if chr == '[' {
            self.read_list()
        } else {
            self.read_typed_value()
        }
    }

    fn has_separator(&mut self) -> anyhow::Result<bool> {
        self.reader.skip_whitespace()?;

        Ok(if !self.reader.done() && self.reader.peek()? == ',' {
            self.reader.skip();
            self.reader.skip_whitespace()?;

            true
        } else {
            false
        })
    }

    pub fn read_struct(&mut self) -> anyhow::Result<Tag> {
        self.expect('{')?;
        self.reader.skip_whitespace()?;

        let mut m = Map::new();

        while self.reader.peek()? != '}' {
            let start = self.reader.position();
            let key = self.read_key()?;

            if key.is_empty() {
                self.reader.set_position(start);
                anyhow::bail!("missing expected key");
            }

            self.expect(':')?;

            // map key to read value
            m.insert(key, self.read_value()?);

            if !self.has_separator()? {
                break;
            }

            if self.reader.done() {
                anyhow::bail!("expected struct closure");
            }
        }

        self.expect('}')?;
        Ok(Tag::Compound(m))
    }

    pub fn read_array_tag(&mut self) -> anyhow::Result<Tag> {
        self.expect('[')?;

        // store start position in case error
        let start = self.reader.position();
        let kind = self.reader.read()?;

        self.reader.skip();
        self.reader.skip_whitespace()?;

        if self.reader.done() {
            self.reader.set_position(start);
            anyhow::bail!("missing expected array value");
        }

        let tag = if kind == 'B' {
            read_array!(self, Byte, i8)
        } else if kind == 'L' {
            read_array!(self, Long, i64)
        } else if kind == 'I' {
            read_array!(self, Int, i32)
        } else {
            anyhow::bail!("unknown array type");
        };

        self.expect(']')?;
        Ok(tag)
    }


    pub fn read_list_tag(&mut self) -> anyhow::Result<Tag> {
        self.expect('[')?;
        self.reader.skip_whitespace()?;

        if self.reader.done() {
            anyhow::bail!("missing expected array value");
        }

        let mut kind = Kind::End;
        let mut v = vec![];

        while self.reader.peek()? != ']' {
            let start = self.reader.position();
            let tag = self.read_value()?;

            if kind == Kind::End {
                kind = tag.kind();
            } else if kind != tag.kind() {
                self.reader.set_position(start);
                anyhow::bail!("array has mixed types");
            }

            v.push(tag);

            if !self.has_separator()? {
                break;
            }

            if self.reader.done() {
                anyhow::bail!("expected array closure")
            }
        }

        self.expect(']')?;
        Ok(Tag::List(v))
    }

    pub fn read_list(&mut self) -> anyhow::Result<Tag> {
        if self.reader.has_remaining(3) && !Reader::is_quote(self.reader.peek_nth(1)?) && self.reader.peek_nth(2)? == ';' {
            self.read_array_tag()
        } else {
            self.read_list_tag()
        }
    }
}

impl Nbt {
    pub fn parse<S: Into<String>>(s: S) -> anyhow::Result<Tag> {
        let mut parser = Parser::new(s);
        let tag = parser.read_value()?;

        Ok(tag)
    }
}
