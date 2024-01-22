use serde::{
  de::{self, DeserializeSeed, MapAccess, Visitor},
  Deserialize,
};

use super::error::{Error, Result};

pub struct Deserializer<'de> {
  // This string starts with the input data and characters are truncated off
  // the beginning as data is parsed.
  input: &'de str,
}

impl<'de> Deserializer<'de> {
  // By convention, `Deserializer` constructors are named like `from_xyz`.
  // That way basic use cases are satisfied by something like
  // `serde_json::from_str(...)` while advanced use cases that require a
  // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(input: &'de str) -> Self {
    Deserializer { input }
  }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
  T: Deserialize<'a>,
{
  let mut deserializer = Deserializer::from_str(s);
  let t = T::deserialize(&mut deserializer)?;
  if deserializer.input.is_empty() {
    Ok(t)
  } else {
    Err(Error::Eof)
  }
}

impl<'de> Deserializer<'de> {
  // Look at the first character in the input without consuming it.
  fn peek_char(&mut self) -> Option<char> {
    self.input.chars().next()
  }

  // Consume the first character in the input.
  fn eat_ch(&mut self, ch: char) {
    self.input = &self.input[ch.len_utf8()..];
  }

  fn parse_bool(&mut self) -> Result<bool> {
    if self.input.starts_with("true") {
      self.input = &self.input["true".len()..];
      Ok(true)
    } else if self.input.starts_with("false") {
      self.input = &self.input["false".len()..];
      Ok(false)
    } else {
      Err(Error::ExpectedBoolean)
    }
  }

  fn parse_string(&mut self) -> Result<&'de str> {
    match self.peek_char() {
      Some(ch) if ch.is_alphanumeric() => {}
      _ => {
        return Err(Error::ExpectedString);
      }
    };
    match self.input.find(|ch: char| !ch.is_alphanumeric()) {
      Some(end) => {
        let st = &self.input[..end];
        self.input = &self.input[end..];
        Ok(st)
      }
      None => {
        let st = self.input;
        self.input = "";
        Ok(st)
      }
    }
  }

  fn parse_f64(&mut self) -> Result<f64> {
    let mut curr_string = String::new();

    match self.peek_char() {
      Some(ch) if ch.is_numeric() => {}
      _ => {
        return Err(Error::ExpectedInteger);
      }
    };

    loop {
      match self.peek_char() {
        Some(ch) if ch.is_numeric() => {
          self.eat_ch(ch);
          curr_string += &ch.to_string()
        }
        _ => return Ok(curr_string.parse::<f64>().unwrap()),
      }
    }
  }
}

// aaa=1&bbb=2
#[test]
fn test_parse_string() {
  let mut de = Deserializer { input: "hello=123" };
  let string = de.parse_string().unwrap();
  assert_eq!(string, "hello");
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
  type Error = Error;

  // Look at the input data to decide what Serde data model type to
  // deserialize as. Not all data formats are able to support this operation.
  // Formats that support `deserialize_any` are known as self-describing.
  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    if let Some(peek_ch) = self.peek_char() {
      match peek_ch {
        'a'..='z' => self.deserialize_str(visitor),
        // '=' => {
        //   self.should_be_value = true;
        // }
        _ => Err(Error::Syntax),
      }
    } else {
      Err(Error::Eof)
    }
  }

  fn deserialize_bool<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_bool(self.parse_bool()?)
  }

  fn deserialize_i8<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i16<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i32<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i64<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u8<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u16<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u32<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u64<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_f32<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_f64<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    visitor.visit_f64(self.parse_f64()?)
  }

  fn deserialize_char<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    visitor.visit_borrowed_str(self.parse_string()?)
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
  where
    V: Visitor<'de>,
  {
    self.deserialize_str(visitor)
  }

  fn deserialize_bytes<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_byte_buf<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_option<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_unit<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_unit_struct<V>(
    self,
    _name: &'static str,
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_newtype_struct<V>(
    self,
    _name: &'static str,
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_seq<V>(self, _visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_tuple<V>(
    self,
    _len: usize,
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_tuple_struct<V>(
    self,
    _name: &'static str,
    _len: usize,
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_map<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    let value = visitor.visit_map(CommaSeparated::new(self))?;
    // Parse the closing brace of the map.
    match self.peek_char() {
      Some(ch) => {
        if ch == '&' {
          Ok(value)
        } else {
          Err(Error::Message("Map End".to_string()))
        }
      }
      None => Ok(value),
    }
  }

  fn deserialize_struct<V>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.deserialize_map(visitor)
  }

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_identifier<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    self.deserialize_str(visitor)
  }

  fn deserialize_ignored_any<V>(
    self,
    _visitor: V,
  ) -> std::prelude::v1::Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    todo!()
  }
}

struct CommaSeparated<'a, 'de: 'a> {
  de: &'a mut Deserializer<'de>,
  need_value: bool,
  need_key: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
  fn new(de: &'a mut Deserializer<'de>) -> Self {
    CommaSeparated {
      de,
      need_value: false,
      need_key: true,
    }
  }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
  where
    K: DeserializeSeed<'de>,
  {
    if !self.need_key {
      return Err(Error::Message("need key".to_string()));
    }

    if let Some(ch) = self.de.peek_char() {
      if ch.is_alphabetic() {
        self.need_value = true;
        let res = seed.deserialize(&mut *self.de).map(Some);
        if let Some(ch) = self.de.peek_char() {
          if ch == '=' {
            self.de.input = &self.de.input[1..];
            self.need_value = true;
          }
        }
        res
      } else {
        Ok(None)
      }
    } else {
      Ok(None)
    }
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
  where
    V: DeserializeSeed<'de>,
  {
    if !self.need_value {
      return Err(Error::Message("need value".to_string()));
    }

    if let Some(ch) = self.de.peek_char() {
      if !ch.is_alphanumeric() {
        return Err(Error::Message("Value should be f64 or String".to_string()));
      }
    }

    let res = seed.deserialize(&mut *self.de);
    self.need_value = false;

    if let Some(ch) = self.de.peek_char() {
      if ch == '&' {
        self.de.input = &self.de.input[1..];
        self.need_key = true;
      }
    }
    res
  }
}
#[allow(unused_imports)]
mod tests {
  use super::*;
  use serde::Deserialize;

  #[test]
  fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
      hello: f64,
      hhh: f64,
    }

    let j = r#"1"#;
    assert_eq!(1_f64, from_str(j).unwrap());
    let j = r#"hello"#;
    assert_eq!("hello", from_str::<&str>(j).unwrap());
    let j = r#"hello=1&hhh=2"#;
    assert_eq!(
      Test {
        hello: 1_f64,
        hhh: 2_f64
      },
      from_str::<Test>(j).unwrap()
    );
  }

  #[test]
  fn test_str() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
      hello: f64,
      hhh: String,
      hh: f64,
    }
    let j = r#"hello=1&hhh=hh&hh=1222"#;
    assert_eq!(
      Test {
        hello: 1_f64,
        hhh: "hh".to_string(),
        hh: 1222_f64
      },
      from_str::<Test>(j).unwrap()
    );
  }
}
