#![recursion_limit = "512"]
#![allow(clippy::unit_arg)]
#![feature(os_string_pathbuf_leak)]

pub(crate) mod binary;
pub mod instance;
pub(crate) mod ipc;
pub mod protocol;

pub mod utils {
    use std::io::BufRead;

    #[extension_traits::extension(pub trait ReadToStringExt)]
    impl<T: BufRead> T {
        fn read_to_new_string(&mut self) -> std::io::Result<String> {
            let mut out = String::new();
            self.read_to_string(&mut out).map(|_| out)
        }
    }
}

#[cfg(test)]
pub mod tests;
