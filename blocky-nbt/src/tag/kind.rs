#[derive(Debug, PartialEq)]
pub enum Kind {
    End,
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
            Self::End => 0,
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