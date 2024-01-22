use std::fmt::{self, Display};

use serde::{de, ser};
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  // One or more variants that can be created by data structures through the
  // `ser::Error` and `de::Error` traits. For example the Serialize impl for
  // Mutex<T> might return an error because the mutex is poisoned, or the
  // Deserialize impl for a struct may return an error because a required
  // field is missing.
  Message(String),

  // Zero or more variants that can be created directly by the Serializer and
  // Deserializer without going through `ser::Error` and `de::Error`. These
  // are specific to the format, in this case JSON.
  Eof,
  Syntax,
  ExpectedBoolean,
  ExpectedInteger,
  ExpectedString,
  ExpectedNull,
}

impl ser::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Message(msg.to_string())
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Message(msg) => write!(f, "{}", msg),
      Error::Eof => f.write_str("unexpected end of input"),
      /* and so forth */
      _ => unimplemented!(),
    }
  }
}

impl std::error::Error for Error {}
