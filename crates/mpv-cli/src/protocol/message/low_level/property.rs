#![allow(dead_code)]

use {
    serde::{Serialize, de::DeserializeOwned},
    serde_json::{Value, json},
    std::collections::HashMap,
};

#[macro_use]
mod codegen;
pub use codegen::*;

pub trait SetPropertyValue {
    type Kind: PropertyValueName<Value = Self>;
    type Primitive: Serialize + DeserializeOwned;
    fn inner(&self) -> &Self::Primitive;
    fn to_json_array(&self) -> Vec<Value> {
        vec![json!(Self::Kind::as_str()), json!(self.inner())]
    }
}

pub trait PropertyValueName {
    type Value: SetPropertyValue + DeserializeOwned;
    fn as_str() -> &'static str;
}

pub mod fields;
