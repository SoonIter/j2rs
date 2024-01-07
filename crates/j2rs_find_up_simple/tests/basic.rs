use j2rs_find_up_simple::find_up;
use std::path::Path;
use sugar_path::SugarPath;

#[test]
fn basic_find_in_current_directory() {
  let left = find_up("Cargo.toml").unwrap();

  let cwd_buf = std::env::current_dir().unwrap();
  let right = cwd_buf.join("Cargo.toml");
  assert_eq!(left, right);
}

#[test]
fn basic_find_in_parent_directory() {
  let left = find_up(Path::new("Cargo.lock")).unwrap();
  let mut right = std::env::current_dir().unwrap();
  right.push(Path::new("../../Cargo.lock"));
  assert_eq!(left, right.normalize());
}

#[test]
fn same_with_lets_find_up() {
  let left = find_up("Cargo.lock").unwrap();
  let right = lets_find_up::find_up("Cargo.lock").unwrap().unwrap();
  assert_eq!(left, right);
}
