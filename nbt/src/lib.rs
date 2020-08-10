use flate2::read::GzDecoder;
use bytes::{Bytes, Buf};
use std::collections::HashMap;
use std::io::Read;
use std::str;
use std::ops;

#[derive(Debug, PartialEq)]
pub enum Kind {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List(u8),
    Compound,
    IntArray,
    LongArray,
}

impl Kind {
    pub fn id(&self) -> u8 {
        match self {
            Self::Byte => 1,
            Self::Short => 2,
            Self::Int => 3,
            Self::Long => 4,
            Self::Float => 5,
            Self::Double => 6,
            Self::ByteArray => 7,
            Self::String => 8,
            Self::List(_) => 9,
            Self::Compound => 10,
            Self::IntArray => 11,
            Self::LongArray => 12,
        }
    }
}

impl Into<u8> for Kind {
    fn into(self) -> u8 {
        self.id()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Payload {
    Null,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(u8, Vec<Self>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

macro_rules! payload {
    ($input:ty, $output:ident) => {
        impl Into<Payload> for $input {
            fn into(self) -> Payload {
                Payload::$output(self)
            }
        }

        impl Into<$input> for Payload {
            fn into(self) -> $input {
                match self {
                    Payload::$output(value) => value,
                    _ => panic!("cannot convert payload"),
                }
            }
        }

        impl<'b> Into<$input> for &'b Payload {
            fn into(self) -> $input {
                match self {
                    Payload::$output(value) => value.to_owned(),
                    _ => panic!("cannot convert payload"),
                }
            }
        }
    };
}

macro_rules! payloads {
    { $({ $input:ty, $output:ident }),* $(,)* } => {
        $(payload!($input, $output);)*
    };
}

payloads! {
    { i8, Byte },
    { i16, Short },
    { i32, Int },
    { i64, Long },

    { f32, Float },
    { f64, Double },

    { Vec<i8>, ByteArray },
    { Vec<i32>, IntArray },
    { Vec<i64>, LongArray },
}

impl Payload {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Null => panic!("null tag does not have a kind"),
            Self::Byte(_) => Kind::Byte,
            Self::Short(_) => Kind::Short,
            Self::Int(_) => Kind::Int,
            Self::Long(_) => Kind::Long,
            Self::Float(_) => Kind::Float,
            Self::Double(_) => Kind::Double,
            Self::ByteArray(_) => Kind::ByteArray,
            Self::String(_) => Kind::String,
            Self::List(id, _) => Kind::List(*id),
            Self::Compound(_) => Kind::Compound,
            Self::IntArray(_) => Kind::IntArray,
            Self::LongArray(_) => Kind::LongArray,
        }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Self> {
        index.index(self)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Self> {
        index.index_mut(self)
    }

    pub fn insert<I: Index>(&mut self, index: I, value: Self) {
        *index.insert(self) = value;
    }
}

pub trait Index {
    fn index<'a>(&self, payload: &'a Payload) -> Option<&'a Payload>;
    fn index_mut<'a>(&self, payload: &'a mut Payload) -> Option<&'a mut Payload>;
    fn insert<'a>(&self, payload: &'a mut Payload) -> &'a mut Payload;
}

impl Index for str {
    fn index<'a>(&self, payload: &'a Payload) -> Option<&'a Payload> {
        match payload {
            Payload::Compound(m) => m.get(self).map(|tag| &tag.payload),
            _ => None,
        }
    }

    fn index_mut<'a>(&self, payload: &'a mut Payload) -> Option<&'a mut Payload> {
        match payload {
            Payload::Compound(m) => m.get_mut(self).map(|tag| &mut tag.payload),
            _ => None,
        }
    }

    fn insert<'a>(&self, payload: &'a mut Payload) -> &'a mut Payload {
        match payload {
            Payload::Compound(m) => {
                let key = self.to_owned();

                &mut m.entry(key.clone())
                    .or_insert(Tag::new(
                        key.clone(),
                        Payload::Null,
                    ))
                    .payload
            },
            _ => panic!("cannot index payload type: {:?}", payload.kind()),
        }
    }
}

impl Index for String {
    fn index<'a>(&self, payload: &'a Payload) -> Option<&'a Payload> {
        self[..].index(payload)
    }

    fn index_mut<'a>(&self, payload: &'a mut Payload) -> Option<&'a mut Payload> {
        self[..].index_mut(payload)
    }

    fn insert<'a>(&self, payload: &'a mut Payload) -> &'a mut Payload {
        self[..].insert(payload)
    }
}

impl Index for usize {
    fn index<'a>(&self, payload: &'a Payload) -> Option<&'a Payload> {
        match payload {
            Payload::List(_, v) => v.get(*self),
            _ => None,
        }
    }

    fn index_mut<'a>(&self, payload: &'a mut Payload) -> Option<&'a mut Payload> {
        match payload {
            Payload::List(_, v) => v.get_mut(*self),
            _ => None,
        }
    }

    fn insert<'a>(&self, payload: &'a mut Payload) -> &'a mut Payload {
        match payload {
            Payload::List(_, v) => {
                let len = v.len();

                v.get_mut(*self)
                    .unwrap_or_else(|| panic!(
                        "cannot access index {} in array of length {}",
                        self,
                        len,
                    ))
            },
            _ => panic!("cannot access index of payload type: {:?}", payload.kind()),
        }
    }
}

impl<'b, T: ?Sized + Index> Index for &'b T {
    fn index<'a>(&self, payload: &'a Payload) -> Option<&'a Payload> {
        (**self).index(payload)
    }

    fn index_mut<'a>(&self, payload: &'a mut Payload) -> Option<&'a mut Payload> {
        (**self).index_mut(payload)
    }

    fn insert<'a>(&self, payload: &'a mut Payload) -> &'a mut Payload {
        (**self).insert(payload)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
    pub payload: Payload,
}

impl Tag {
    pub fn new(name: String, payload: Payload) -> Self {
        Self { name, payload }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Payload> {
        self.payload.get(index)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Payload> {
        self.payload.get_mut(index)
    }

    pub fn insert<I: Index>(&mut self, index: I, value: Payload) {
        self.payload.insert(index, value);
    }

    fn decode_payload<T: Buf>(buf: &mut T, id: u8) -> anyhow::Result<Payload> {
        match id {
            0 => Ok(Payload::Null),

            1 => Ok(Payload::Byte(buf.get_i8())),
            2 => Ok(Payload::Short(buf.get_i16())),
            3 => Ok(Payload::Int(buf.get_i32())),
            4 => Ok(Payload::Long(buf.get_i64())),

            5 => Ok(Payload::Float(buf.get_f32())),
            6 => Ok(Payload::Double(buf.get_f64())),

            8 => {
                let len = buf.get_i16() as usize;
                let s = match str::from_utf8(&buf.bytes()[0..len]) {
                    Ok(s) => s.to_owned(),
                    Err(_) => anyhow::bail!("failed to decode string"),
                };

                // skip string bytes
                buf.advance(len);

                // return string tag
                Ok(Payload::String(s))
            },

            9 => {
                let tag_id = buf.get_u8();
                let len = buf.get_i32();
                let mut v = vec![];

                for _ in 0..len {
                    v.push(Self::decode_payload(buf, tag_id)?);
                }

                Ok(Payload::List(tag_id, v))
            },

            10 => {
                let mut m = HashMap::new();

                loop {
                    let tag = Self::decode_tag(buf)?;

                    if tag.payload == Payload::Null {
                        break;
                    }

                    m.insert(tag.name.clone(), tag);
                }

                Ok(Payload::Compound(m))
            },

            7 => {
                let len = buf.get_i32();
                let mut v = vec![];

                for _ in 0..len {
                    v.push(buf.get_i8());
                }

                Ok(Payload::ByteArray(v))
            },
            11 => {
                let len = buf.get_i32();
                let mut v = vec![];

                for _ in 0..len {
                    v.push(buf.get_i32());
                }

                Ok(Payload::IntArray(v))
            },
            12 => {
                let len = buf.get_i32();
                let mut v = vec![];

                for _ in 0..len {
                    v.push(buf.get_i64());
                }

                Ok(Payload::LongArray(v))
            },

            _ => anyhow::bail!("unknown id: {}", id),
        }
    }

    fn decode_tag<T: Buf>(buf: &mut T) -> anyhow::Result<Tag> {
        let id = buf.get_u8();

        let name_len = if id != 0 {
            buf.get_u16() as usize
        } else {
            0
        };

        let name = if name_len > 0 {
            let s = match str::from_utf8(&buf.bytes()[0..name_len]) {
                Ok(s) => s.to_owned(),
                Err(_) => anyhow::bail!("failed to decode name"),
            };

            // skip name string bytes
            buf.advance(name_len);

            // return name
            s
        } else {
            String::new()
        };

        // decode tag payload
        let payload = Self::decode_payload(buf, id)?;

        // return tag with header
        Ok(Self::new(name, payload ))
    }

    pub fn decode<R: Read>(src: &mut R) -> anyhow::Result<Self> {
        // decode gz compression
        let mut buf = vec![];

        src.read_to_end(&mut buf)?;

        // try to decompress source
        {
            let mut decompressed = vec![];
            let mut gz = GzDecoder::new(&buf[..]);

            // change buffer if decompression worked
            if let Ok(_) = gz.read_to_end(&mut decompressed) {
                buf = decompressed;
            }
        }

        let mut bytes = Bytes::copy_from_slice(&buf);

        // return tag
        Self::decode_tag(&mut bytes)
    }
}

impl<I: Index> ops::Index<I> for Payload {
    type Output = Self;

    fn index(&self, index: I) -> &Self::Output {
        match self.get(index) {
            Some(payload) => payload,
            None => &Payload::Null,
        }
    }
}

impl<I: Index> ops::IndexMut<I> for Payload {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.insert(self)
    }
}

impl ops::Index<&str> for Tag {
    type Output = Payload;

    fn index<'a>(&self, index: &str) -> &Self::Output {
        &self.payload[index]
    }
}

impl ops::IndexMut<&str> for Tag {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut self.payload[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Kind, Tag};
    use std::io::Cursor;

    fn level(data: &[u8]) {
        let data = data.to_vec();
        let mut data = Cursor::new(data);

        // decode nbt from data
        let nbt = Tag::decode(&mut data);

        assert!(nbt.is_ok(), "failed to decode nbt");

        // unwrap nbt data
        let nbt = nbt.unwrap();

        assert!(match nbt.payload.kind() {
            Kind::Compound => true,
            _ => false,
        }, "nbt does not match expected type");
    }

    #[test]
    fn uncompressed_level() {
        level(include_bytes!("../examples/uncompressed.nbt"));
    }

    #[test]
    fn compressed_level() {
        level(include_bytes!("../examples/compressed.nbt"));
    }
}
