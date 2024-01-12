use std::path::{Path, PathBuf};

use j2rs_find_up_simple::{find_up, find_up_with, FindUpOptions};

pub fn package_up_with(cwd: &Path) -> Option<PathBuf> {
  find_up_with(
    "package.json",
    FindUpOptions {
      cwd,
      ..Default::default()
    },
  )
}

pub fn package_up() -> Option<PathBuf> {
  find_up("package.json")
}
