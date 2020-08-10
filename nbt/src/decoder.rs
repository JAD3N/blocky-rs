use crate::{Nbt, Tag};
use flate2::read::GzDecoder;
use bytes::{Bytes, Buf};
use linked_hash_map::LinkedHashMap;
use std::io::Read;
use std::str;

fn decode_tag(buf: &mut Bytes, id: u8) -> anyhow::Result<Tag> {
    match id {
        0 => Ok(Tag::End),

        1 => Ok(Tag::Byte(buf.get_i8())),
        2 => Ok(Tag::Short(buf.get_i16())),
        3 => Ok(Tag::Int(buf.get_i32())),
        4 => Ok(Tag::Long(buf.get_i64())),

        5 => Ok(Tag::Float(buf.get_f32())),
        6 => Ok(Tag::Double(buf.get_f64())),

        8 => {
            let len = buf.get_i16() as usize;
            let s = match str::from_utf8(&buf.bytes()[0..len]) {
                Ok(s) => s.to_owned(),
                Err(_) => anyhow::bail!("failed to decode string"),
            };

            // skip string bytes
            buf.advance(len);

            // return string nbt
            Ok(Tag::String(s))
        },

        9 => {
            let nbt_id = buf.get_u8();
            let len = buf.get_i32();
            let mut v = vec![];

            for _ in 0..len {
                v.push(decode_tag(buf, nbt_id)?);
            }

            Ok(Tag::List(nbt_id, v))
        },

        10 => {
            let mut m = LinkedHashMap::new();

            loop {
                let nbt = decode_nbt(buf)?;

                if nbt.tag == Tag::End {
                    break;
                }

                m.insert(nbt.name, nbt.tag);
            }

            Ok(Tag::Compound(m))
        },

        7 => {
            let len = buf.get_i32();
            let mut v = vec![];

            for _ in 0..len {
                v.push(buf.get_i8());
            }

            Ok(Tag::ByteArray(v))
        },
        11 => {
            let len = buf.get_i32();
            let mut v = vec![];

            for _ in 0..len {
                v.push(buf.get_i32());
            }

            Ok(Tag::IntArray(v))
        },
        12 => {
            let len = buf.get_i32();
            let mut v = vec![];

            for _ in 0..len {
                v.push(buf.get_i64());
            }

            Ok(Tag::LongArray(v))
        },

        _ => anyhow::bail!("unknown id: {}", id),
    }
}

fn decode_nbt(buf: &mut Bytes) -> anyhow::Result<Nbt> {
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

    // decode nbt tag
    let tag = decode_tag(buf, id)?;

    // return nbt with header
    Ok(Nbt::new(name, tag ))
}

impl Nbt {
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

        // return nbt
        decode_nbt(&mut bytes)
    }
}
