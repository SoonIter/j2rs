use std::borrow::Cow;

pub use ansi_regex::ansi_regex;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  pub static ref ANSI_REGEX: Regex = ansi_regex();
}

pub fn strip_ansi(st: &str) -> Cow<'_, str> {
  let replaced_str = ANSI_REGEX.replace_all(st, "");
  replaced_str
}
