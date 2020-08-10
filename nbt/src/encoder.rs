use crate::{Nbt, Tag, Kind};
use flate2::write::GzEncoder;
use flate2::Compression;
use bytes::{BytesMut, BufMut};
use std::io::Write;

fn encode_tag(buf: &mut BytesMut, tag: &Tag) -> anyhow::Result<()> {
    match tag {
        Tag::End => panic!("cannot encode null"),

        Tag::Byte(value) => {
            buf.reserve(1);
            buf.put_i8(*value);
        },
        Tag::Short(value) => {
            buf.reserve(2);
            buf.put_i16(*value);
        },
        Tag::Int(value) => {
            buf.reserve(4);
            buf.put_i32(*value);
        },
        Tag::Long(value) => {
            buf.reserve(8);
            buf.put_i64(*value);
        },

        Tag::Float(value) => {
            buf.reserve(4);
            buf.put_f32(*value);
        },
        Tag::Double(value) => {
            buf.reserve(8);
            buf.put_f64(*value);
        },

        Tag::String(value) => {
            let value = value.as_bytes();
            let len = value.len();

            buf.reserve(2 + len);
            buf.put_i16(len as i16);
            buf.put_slice(value);
        },

        Tag::List(nbt_id, v) => {
            let len = v.len() as i32;

            buf.reserve(5);
            buf.put_u8(*nbt_id);
            buf.put_i32(len);

            for tag in v {
                encode_tag(buf, tag)?;
            }
        },

        Tag::Compound(m) => {
            for (name, tag)  in m {
                if *tag != Tag::End {
                    encode_nbt(buf, name, tag)?;
                }
            }

            buf.reserve(1);
            buf.put_u8(Kind::End.id());
        },

        Tag::ByteArray(v) => {
            let len = v.len();

            buf.reserve(4 + len);
            buf.put_i32(len as i32);

            for value in v {
                buf.put_i8(*value);
            }
        },
        Tag::IntArray(v) => {
            let len = v.len();

            buf.reserve(4 + len * 4);
            buf.put_i32(len as i32);

            for value in v {
                buf.put_i32(*value);
            }
        },
        Tag::LongArray(v) => {
            let len = v.len();

            buf.reserve(4 + len * 8);
            buf.put_i32(len as i32);

            for value in v {
                buf.put_i64(*value);
            }
        },
    }

    Ok(())
}

fn encode_nbt(buf: &mut BytesMut, name: &str, tag: &Tag) -> anyhow::Result<()> {
    let id = tag.kind().id();
    let name = name.as_bytes();
    let name_len = name.len();

    // reserve id and name
    buf.reserve(1 + name_len * 2);

    // add nbt id
    buf.put_u8(id);

    // add name length and contents
    buf.put_u16(name_len as u16);
    buf.put_slice(name);

    encode_tag(buf, &tag)?;

    Ok(())
}

impl Nbt {
    pub fn encode<W: Write>(&self, dst: &mut W, compress: bool) -> anyhow::Result<()> {
        let mut bytes = BytesMut::new();

        // encode nbts into uncompressed bytes
        encode_nbt(&mut bytes, &self.name, &self.tag)?;

        if compress {
            // create encoder
            let mut gz = GzEncoder::new(dst, Compression::default());

            // compress bytes
            gz.write_all(&bytes)?;
        } else {
            dst.write_all(&bytes)?;
        }

        Ok(())
    }
}
