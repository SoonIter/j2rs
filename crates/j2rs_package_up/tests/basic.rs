use j2rs_package_up::package_up_with;

#[test]
fn basic_package_up_in_current_directory() {
  let cwd_buf = std::env::current_dir().unwrap();
  let cwd = cwd_buf.join("fixtures/basic");
  let left = package_up_with(&cwd).unwrap();
  let right = cwd.join("package.json");

  assert_eq!(left, right);
}

#[test]
fn basic_package_up_in_parent_directory() {
  let cwd_buf = std::env::current_dir().unwrap();
  let cwd = cwd_buf.join("fixtures/basic/src/nested/cwd");
  let left = package_up_with(&cwd).unwrap();
  let right = cwd_buf.join("fixtures/basic/package.json");

  assert_eq!(left, right);
}
