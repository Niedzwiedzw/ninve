#![recursion_limit = "512"]
#![allow(clippy::unit_arg)]
#![feature(os_string_pathbuf_leak)]

pub mod binary;
pub mod instance;
pub mod ipc;
pub mod protocol;

pub mod utils;

#[cfg(test)]
pub mod tests;
