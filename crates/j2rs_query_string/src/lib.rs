use serde::de::DeserializeOwned;
use std::fmt::Debug;
mod parser;
use parser::de::from_str;
use parser::error::Result;

pub fn try_parse<T>(qs: impl AsRef<str>) -> Result<T>
where
  T: DeserializeOwned + PartialEq + Debug,
{
  let slice = qs.as_ref();
  from_str(slice)
}

pub fn parse<T>(qs: impl AsRef<str>) -> T
where
  T: DeserializeOwned + PartialEq + Debug,
{
  try_parse(qs).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_parse_check<T>(query: &str, value: T)
  where
    T: DeserializeOwned + PartialEq + Debug,
  {
    assert_eq!(parse::<T>(query), value);
  }

  #[test]
  fn test_parse() {
    use serde::Deserialize;
    #[derive(Debug, PartialEq, Deserialize)]
    struct Pagination {
      size: f64,
      page: f64,
    }

    assert_parse_check(
      "size=1&page=2",
      Pagination {
        size: 1_f64,
        page: 2_f64,
      },
    );
  }
}
