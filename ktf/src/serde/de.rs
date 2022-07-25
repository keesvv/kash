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
            '0'..='9' => self.deserialize_f32(visitor),
            '|' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.next_char()? == '|' {
            let val = visitor.visit_map(StructMap::new(self));
            Ok(val?)
        } else {
            Err(Error::ExpectedMap)
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct struct enum identifier ignored_any
    }
}

struct StructMap<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> StructMap<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de, first: true }
    }
}

impl<'a, 'de> MapAccess<'de> for StructMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Err(Error::Eof) = self.de.peek_char() {
            return Ok(None);
        }

        if !self.first && self.de.next_char()? != '|' {
            return Err(Error::ExpectedMapValue);
        }

        self.first = false;

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
