#[cfg(feature = "output-json")]
pub mod json;

use std::io::{Result, Write};

pub trait Output {
    // TODO: custom Result and Error enums which differentiate
    // between I/O and serialization errors.
    fn to_write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write;

    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.to_write(&mut buf).unwrap();
        buf
    }

    fn to_string(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }
}
