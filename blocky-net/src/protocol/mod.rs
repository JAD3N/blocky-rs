use thiserror::Error;
use bytes::{Buf, BufMut};
use crate::FromJson;
use crate::chat::Component;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("not enough bytes remaining")]
    NotEnoughBytes,

    #[error("invalid value")]
    Invalid,

    #[error("value is too large")]
    TooLarge,

    #[error("unknown protocol error")]
    Unknown,
}


pub trait ProtocolLength {
    fn len(&self) -> usize;
}

pub trait ProtocolRead: ProtocolLength {
    fn read<U: Buf>(_src: &mut U) -> Result<Self, ProtocolError> where Self: Sized {
        unimplemented!("protocol read not implemented");
    }
}

pub trait ProtocolWrite: ProtocolLength {
    fn write<U: BufMut>(&self, _dst: &mut U) -> Result<(), ProtocolError> where Self: Sized {
        unimplemented!("protocol write not implemented");
    }
}

#[macro_export]
macro_rules! protocol_struct {
    ($name:ident { $($fname:ident: $fty:ty),* $(,)? }) => {
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl $crate::protocol::ProtocolLength for $name {
            fn len(&self) -> usize {
                0 $(+ self.$fname.len())*
            }
        }

        impl $crate::protocol::ProtocolRead for $name {
            fn read<U: bytes::Buf>(_src: &mut U) -> Result<Self, $crate::protocol::ProtocolError> {
                Ok($name { $($fname: <$fty>::read(_src)?,)* })
            }
        }

        impl $crate::protocol::ProtocolWrite for $name {
            fn write<U: bytes::BufMut>(&self, _dst: &mut U) -> Result<(), $crate::protocol::ProtocolError> {
                $(self.$fname.write(_dst)?;)*
                Ok(())
            }
        }
    };
}

macro_rules! protocol_data_scalar {
    ($t:ty, $len:expr, $w:ident, $r:ident) => {
        impl ProtocolLength for $t {
            fn len(&self) -> usize { $len }
        }

        impl ProtocolRead for $t {
            fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
                if src.remaining() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    Ok(src.$r())
                }
            }
        }

        impl ProtocolWrite for $t {
            fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
                if dst.remaining_mut() < $len {
                    Err(ProtocolError::NotEnoughBytes)
                } else {
                    dst.$w(*self);
                    Ok(())
                }
            }
        }
    };
}

protocol_data_scalar!(u8, 1, put_u8, get_u8);
protocol_data_scalar!(u16, 2, put_u16, get_u16);
protocol_data_scalar!(u32, 4, put_u32, get_u32);
protocol_data_scalar!(u64, 8, put_u64, get_u64);

protocol_data_scalar!(i8, 1, put_i8, get_i8);
protocol_data_scalar!(i16, 2, put_i16, get_i16);
protocol_data_scalar!(i32, 4, put_i32, get_i32);
protocol_data_scalar!(i64, 8, put_i64, get_i64);

protocol_data_scalar!(f32, 4, put_f32, get_f32);
protocol_data_scalar!(f64, 8, put_f64, get_f64);

impl ProtocolLength for bool {
    fn len(&self) -> usize { 1 }
}

impl ProtocolRead for bool {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let value = <u8>::read(src)?;

        if value > 1 {
            Err(ProtocolError::Invalid)
        } else {
            Ok(value == 1)
        }
    }
}

impl ProtocolWrite for bool {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        <u8>::write(&(*self as u8), dst)
    }
}

#[derive(Debug)]
pub struct Var<T>(pub T);
pub type VarInt = Var<i32>;
pub type VarLong = Var<i64>;

impl<T: Into<i32>> From<T> for Var<i32> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl<T: Into<i64>> From<T> for Var<i64> {
    fn from(value: T) -> Self {
        Var(value.into())
    }
}

impl ProtocolLength for Var<i32> {
    fn len(&self) -> usize {
        let mut value = self.0;

        for i in 1..5 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        5
    }
}

impl ProtocolRead for Var<i32> {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let mut value = 0i32;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = <u8>::read(src)? as i32;

            value |= (byte & 0b01111111) << (7 * reads);
            reads += 1;

            if reads > 5 {
                // VarInt is bigger than 5 bytes
                return Err(ProtocolError::TooLarge);
            }

            if byte & 0b10000000 == 0 {
                break;
            }
        }

        Ok(Var(value))
    }
}

impl ProtocolWrite for Var<i32> {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < self.len() {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = self.0;

            loop {
                let mut byte = (value & 0b01111111) as u8;

                value >>= 7;

                if value != 0 {
                    byte |= 0b10000000;
                }

                dst.put_u8(byte);

                if value == 0 {
                    break;
                }
            }

            Ok(())
        }
    }
}

impl ProtocolLength for Var<i64> {
    fn len(&self) -> usize {
        let mut value = self.0;

        for i in 1..10 {
            value >>= 7;

            if value == 0 {
                return i;
            }
        }

        5
    }
}

impl ProtocolRead for Var<i64> {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let mut value = 0i64;
        let mut reads = 0usize;

        loop {
            // do remaining check in u8
            let byte = <u8>::read(src)? as i64;

            value |= (byte & 0b01111111) << (7 * reads);
            reads += 1;

            if reads > 10 {
                // VarInt is bigger than 10 bytes
                return Err(ProtocolError::TooLarge);
            }

            if byte & 0b10000000 == 0 {
                break;
            }
        }

        Ok(Var(value))
    }
}

impl ProtocolWrite for Var<i64> {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        // do remaining check beforehand
        if dst.remaining_mut() < self.len() {
            Err(ProtocolError::NotEnoughBytes)
        } else {
            let mut value = self.0;

            loop {
                let mut byte = (value & 0b01111111) as u8;

                value >>= 7;

                if value != 0 {
                    byte |= 0b10000000;
                }

                dst.put_u8(byte);

                if value == 0 {
                    break;
                }
            }

            Ok(())
        }
    }
}

impl ProtocolLength for String {
    fn len(&self) -> usize {
        let len = String::len(self);
        let len_var: Var<i32> = (len as i32).into();

        len_var.len() + len
    }
}

impl ProtocolRead for String {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let len_var = <Var<i32>>::read(src)?;
        let len = len_var.0 as usize;

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            let mut bytes = vec![0u8; len];

            // copy bytes to vec
            src.copy_to_slice(&mut bytes);

            // convert bytes to string
            String::from_utf8(bytes)
                .map_err(|_| ProtocolError::Invalid)
        }
    }
}

impl ProtocolWrite for String {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        let len = String::len(self);
        let len_var: Var<i32> = (len as i32).into();

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            <Var<i32>>::write(&len_var, dst)?;

            for &byte in self.as_bytes() {
                dst.put_u8(byte);
            }

            Ok(())
        }
    }
}

impl ProtocolLength for Box<dyn Component> {
    fn len(&self) -> usize {
        let json = self.as_json().to_string();
        let len = String::len(&json);
        let len_var: Var<i32> = (len as i32).into();

        len_var.len() + len
    }
}

impl ProtocolRead for Box<dyn Component> {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let len_var = <Var<i32>>::read(src)?;
        let len = len_var.0 as usize;

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            let mut bytes = vec![0u8; len];

            // copy bytes to vec
            src.copy_to_slice(&mut bytes);

            // convert bytes to string
            let s = String::from_utf8(bytes)
                .map_err(|_| ProtocolError::Invalid)?;

            // try convert json into boxed component
            Box::<dyn Component>::from_json(&s.into())
                .map_err(|_| ProtocolError::Invalid)
        }
    }
}

impl ProtocolWrite for Box<dyn Component> {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        let len = Self::len(self);
        let len_var: Var<i32> = (len as i32).into();

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            <Var<i32>>::write(&len_var, dst)?;

            // get json data
            let json = self.as_json().to_string();

            for &byte in json.as_bytes() {
                dst.put_u8(byte);
            }

            Ok(())
        }
    }
}

impl<T: Component> ProtocolLength for T {
    fn len(&self) -> usize {
        let json = self.as_json().to_string();
        let len = String::len(&json);
        let len_var: Var<i32> = (len as i32).into();

        len_var.len() + len
    }
}

impl<T: Component> ProtocolRead for T {
    fn read<U: Buf>(src: &mut U) -> Result<Self, ProtocolError> {
        let c = Box::<dyn Component>::read(src)?;

        if let Ok(c) = c.downcast() {
            Ok(*c)
        } else {
            Err(ProtocolError::Invalid)
        }
    }
}

impl<T: Component> ProtocolWrite for T {
    fn write<U: BufMut>(&self, dst: &mut U) -> Result<(), ProtocolError> {
        let len = Self::len(self);
        let len_var: Var<i32> = (len as i32).into();

        if len > 32767 {
            Err(ProtocolError::TooLarge)
        } else {
            <Var<i32>>::write(&len_var, dst)?;

            // get json data
            let json = self.as_json().to_string();

            for &byte in json.as_bytes() {
                dst.put_u8(byte);
            }

            Ok(())
        }
    }
}

protocol_struct!(Test {
    text: crate::chat::TextComponent,
});