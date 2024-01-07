use std::path::PathBuf;

use j2rs_find_up_simple::{find_up_with, FindUpOptions};
use sugar_path::SugarPath;

#[test]
fn has_option_basic() {
  let cwd_buf = std::env::current_dir().unwrap();

  let right = find_up_with(
    PathBuf::from("Cargo.toml"),
    FindUpOptions {
      cwd: &cwd_buf,
      ..Default::default()
    },
  )
  .unwrap();

  let left = cwd_buf.join("Cargo.toml");

  assert_eq!(left, right);
}

#[test]
fn has_option_up_dir() {
  let cwd_buf = std::env::current_dir().unwrap();

  let right = find_up_with(
    PathBuf::from("Cargo.lock"),
    FindUpOptions {
      cwd: &cwd_buf,
      ..Default::default()
    },
  )
  .unwrap();

  let left = cwd_buf.join("../../Cargo.lock");
  let left = left.absolutize();

  assert_eq!(left, right);
}
