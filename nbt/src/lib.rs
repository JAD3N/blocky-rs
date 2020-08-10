#[macro_use]
extern crate lazy_static;

mod decoder;
mod encoder;
#[macro_use]
mod tag;

pub use tag::*;

use std::str;
use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Nbt {
    pub name: String,
    pub tag: Tag,
}

impl Nbt {
    pub fn new(name: String, tag: Tag) -> Self {
        Self { name, tag }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Tag> {
        self.tag.get(index)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Tag> {
        self.tag.get_mut(index)
    }

    pub fn insert<I: Index>(&mut self, index: I, value: Tag) {
        self.tag.insert(index, value);
    }
}

impl<I: Index> ops::Index<I> for Tag {
    type Output = Self;

    fn index(&self, index: I) -> &Self::Output {
        match self.get(index) {
            Some(tag) => tag,
            None => &Tag::End,
        }
    }
}

impl<I: Index> ops::IndexMut<I> for Tag {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

impl ops::Index<&str> for Nbt {
    type Output = Tag;

    fn index<'a>(&self, index: &str) -> &Self::Output {
        &self.tag[index]
    }
}

impl ops::IndexMut<&str> for Nbt {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut self.tag[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Kind, Nbt, Tag, tag};
    use std::io::Cursor;

    fn nbt(data: &[u8]) -> Nbt {
        let data = data.to_vec();
        let mut data = Cursor::new(data);

        // decode nbt from data
        let nbt = Nbt::decode(&mut data);

        assert!(nbt.is_ok(), "failed to decode nbt");

        // unwrap nbt data
        let nbt = nbt.unwrap();

        assert!(match nbt.tag.kind() {
            Kind::Compound => true,
            _ => false,
        }, "nbt does not match expected type");

        nbt
    }

    #[test]
    fn uncompressed() {
        nbt(include_bytes!("../examples/uncompressed.nbt"));
    }

    #[test]
    fn compressed() {
        nbt(include_bytes!("../examples/compressed.nbt"));
    }

    #[test]
    fn insertion_order() {
        let bytes = include_bytes!("../examples/uncompressed.nbt");
        let nbt = nbt(bytes);
        let mut new_bytes = vec![];

        assert!(nbt.encode(&mut new_bytes, false).is_ok(), "failed to encode");
        assert!(bytes.len() == new_bytes.len(), "size doesn't match");
        assert!({
            let len = bytes.len();
            let mut matches = true;

            for i in 0..len {
                if bytes[i] != new_bytes[i] {
                    matches = false;
                    break;
                }
            }

            matches
        }, "data doesn't match")
    }

    #[test]
    fn string() {
        let nbt: Tag = String::from("\"\"\\'this is a test!").into();
        assert_eq!(format!("{}", nbt), "'\"\"\\\\\\'this is a test!'");
    }

    #[test]
    fn assignment() {
        let mut nbt = nbt(include_bytes!("../examples/uncompressed.nbt"));
        nbt["testing"] = tag!("[B;8b,2b]");

        assert_eq!(nbt["testing"], tag!("[B;8b,2b]"));
    }

    #[test]
    fn display() {
        let byte_array = tag!("[B;1B,2B,6b,10b]").to_string();
        let list = tag!("[5l,10L,20l]").to_string();
        let compound = tag!("{name:'Jaden'}").to_string();

        assert_eq!(byte_array, "[B;1b,2b,6b,10b]");
        assert_eq!(list, "[5L,10L,20L]");
        assert_eq!(compound, "{name:\"Jaden\"}");
    }

    #[test]
    fn verify_value() {
        let nbt = nbt(include_bytes!("../examples/uncompressed.nbt"));
        let seed: i64 = (&nbt["Data"]["RandomSeed"]).into();
        let version_name: String = (&nbt["Data"]["Version"]["Name"]).into();

        assert_eq!(seed, 4443890602994873962);
        assert_eq!(version_name, "1.14.1 Pre-Release 2");
    }
}
