#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod chat;

pub trait AsJson {
    fn as_json(&self) -> serde_json::Value;
}

pub trait FromJson: Sized {
    type Err;
    fn from_json(value: &serde_json::Value) -> Result<Self, Self::Err>;
}
