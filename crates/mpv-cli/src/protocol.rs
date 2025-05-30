use {
    serde::{Serialize, de::DeserializeOwned},
    std::fmt::Debug,
};

pub trait MpvCommand: Serialize + Debug {
    type Response: DeserializeOwned + Debug;
}

pub mod event;
pub mod message;
