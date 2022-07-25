pub mod de;
pub mod error;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "serde")]
pub use self::serde::*;

#[cfg(test)]
mod tests {
    use super::de::Deserializer;
    use super::error::Result;

    #[test]
    fn de_header_single_col() -> Result<()> {
        let mut des = Deserializer::from_str(">col1");
        Ok(assert_eq!(vec!["col1"], des.parse_header()?))
    }
}
