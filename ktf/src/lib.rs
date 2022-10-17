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
        let mut des = Deserializer::from_str(">col1\n");
        Ok(assert_eq!(vec!["col1"], des.parse_header()?))
    }

    #[test]
    fn de_header_multi_col() -> Result<()> {
        let mut des = Deserializer::from_str(">col1|col2|col3\n");
        Ok(assert_eq!(
            vec!["col1", "col2", "col3"],
            des.parse_header()?
        ))
    }
}
