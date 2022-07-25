use crate::de::Deserializer;
use crate::error::{Error, Result};
use serde::de::{self, Deserialize, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            '>' => {
                self.parse_header()?;
                self.deserialize_map(visitor)
            }
            // '0'..='9' => self.deserialize_f32(visitor),
            // '|' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(RowMap::new(self))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct struct enum identifier ignored_any
    }
}

struct RowMap<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    col_index: usize,
}

impl<'a, 'de> RowMap<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de, col_index: 0 }
    }

    pub fn next_col(&mut self) -> Option<String> {
        self.de
            .cur_header
            .iter()
            .nth(self.col_index)
            .map(String::to_owned)
    }
}

impl<'a, 'de> MapAccess<'de> for RowMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let None = self.next_col() {
            return Ok(None);
        }

        // this currently fails because
        // there is no string deserializer implemented.
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
