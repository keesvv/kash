use crate::de::Deserializer;
use crate::error::{Error, Result};
use serde::de::value::StrDeserializer;
use serde::de::{self, Deserialize, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut des = Deserializer::from_str(s);
    des.parse_header()?;

    let t = T::deserialize(&mut des)?;
    Ok(t)
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            '|' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.next_char()? != '|' {
            Err(Error::ExpectedMap)
        } else {
            visitor.visit_map(RowMap::new(self))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(
            self.next_char()?
                .to_string()
                .parse()
                .map_err(|_| Error::ExpectedFloat)?,
        )
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct struct enum identifier ignored_any
    }
}

struct RowMap<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> RowMap<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> MapAccess<'de> for RowMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.de.next_col() {
            None => Ok(None),
            Some(col) => seed.deserialize(StrDeserializer::new(&col)).map(Some),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
