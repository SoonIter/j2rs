#[cfg(test)]
mod tests {
  use j2rs_find_up::find_up;
  use std::path::Path;
  use sugar_path::SugarPath;

  #[test]
  fn basic() {
    let left = find_up("Cargo.toml".into()).unwrap();

    let cwd_buf = std::env::current_dir().unwrap();
    let right = cwd_buf.join("Cargo.toml");
    assert_eq!(left, right);
  }

  #[test]
  fn hello() {
    let left = find_up(Path::new("Cargo.lock").into()).unwrap();
    let mut right = std::env::current_dir().unwrap();
    right.push(Path::new("../../Cargo.lock"));

    assert_eq!(left, right.normalize());
  }
}
