use std::path::PathBuf;

use j2rs_find_up_simple::{find_up_with, FindUpOptions};

fn main() {
  let cwd_buf = std::env::current_dir().unwrap();
  let x = find_up_with(
    PathBuf::from("hello.tt"),
    FindUpOptions {
      cwd: &cwd_buf,
      ..Default::default()
    },
  );
  dbg!(x);
}
