#[macro_use]
extern crate mopa;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

trait Test: Sized {
    fn test(&self) -> Vec<Box<Self>>;
}

pub mod chat;
#[macro_use]
pub mod protocol;

pub trait AsJson {
    fn as_json(&self) -> serde_json::Value;
}

pub trait FromJson {
    type Err;
    fn from_json(value: &serde_json::Value) -> Result<Self, Self::Err> where Self: Sized;
}
