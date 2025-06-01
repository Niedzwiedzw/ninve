use std::io::BufRead;

#[extension_traits::extension(pub trait ReadToStringExt)]
impl<T: BufRead> T {
    fn read_to_new_string(&mut self) -> std::io::Result<String> {
        let mut out = String::new();
        self.read_to_string(&mut out).map(|_| out)
    }
}

pub mod abort_on_drop;
