use std::path::{Path, PathBuf};

pub enum FindUpKind {
  File,
  Directory,
}

pub struct FindUpOptions<'a> {
  pub cwd: &'a Path,
  pub kind: FindUpKind,
  pub stop_at: PathBuf,
}

impl<'a> Default for FindUpOptions<'a> {
  fn default() -> Self {
    Self {
      cwd: Path::new("."),
      kind: FindUpKind::File,
      stop_at: PathBuf::from("/"),
    }
  }
}

pub fn find_up_with<T: AsRef<Path>>(name: T, option: FindUpOptions) -> Option<PathBuf> {
  let FindUpOptions { cwd, kind, stop_at } = option;

  let mut curr_path_name = if cwd.eq(Path::new(".")) {
    std::env::current_dir().unwrap()
  } else {
    PathBuf::from(option.cwd)
  };

  let is_dir_kind = matches!(kind, FindUpKind::Directory);

  loop {
    curr_path_name.push(name.as_ref());

    if !is_dir_kind && curr_path_name.is_file() {
      return Some(curr_path_name);
    }
    if is_dir_kind && curr_path_name.is_dir() {
      return Some(curr_path_name);
    }

    curr_path_name.pop();

    if curr_path_name.eq(&stop_at) {
      return None;
    }
    if curr_path_name.eq(Path::new("/")) {
      return None;
    }

    curr_path_name.pop();
  }
}

#[inline]
pub fn find_up<T: AsRef<Path>>(name: T) -> Option<PathBuf> {
  find_up_with(name, FindUpOptions::default())
}
