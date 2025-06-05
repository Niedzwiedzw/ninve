#![recursion_limit = "512"]
#![allow(clippy::unit_arg)]

pub mod binary;
pub mod instance;
pub mod ipc;
pub mod protocol;

pub mod utils;

#[cfg(test)]
pub mod tests;
