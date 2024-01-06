use std::path::{Path, PathBuf};

pub enum FindUpKind {
  File,
  Directory,
}

pub struct FindUpOption {
  cwd: PathBuf,
  kind: FindUpKind,
  stop_at: PathBuf,
}

impl Default for FindUpOption {
  fn default() -> Self {
    let cwd_buf = std::env::current_dir().unwrap();
    Self {
      cwd: cwd_buf,
      kind: FindUpKind::File,
      stop_at: PathBuf::new().join("/"),
    }
  }
}

pub fn find_up_with(name: PathBuf, option: FindUpOption) -> Option<PathBuf> {
  let FindUpOption { cwd, kind, stop_at } = option;
  let mut curr_path_name = cwd;

  let is_dir_kind = matches!(kind, FindUpKind::Directory);

  loop {
    let temp_curr_path_name = curr_path_name.join(&name);

    if !is_dir_kind && temp_curr_path_name.is_file() {
      return Some(temp_curr_path_name);
    }
    if is_dir_kind && temp_curr_path_name.is_dir() {
      return Some(temp_curr_path_name);
    }
    if temp_curr_path_name.eq(&stop_at) || temp_curr_path_name.eq(Path::new("/")) {
      return None;
    }

    curr_path_name.pop();
  }
}

pub fn find_up(name: PathBuf) -> Option<PathBuf> {
  find_up_with(name, FindUpOption::default())
}
