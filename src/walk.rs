use std::{
  path::PathBuf,
  io,
};

#[derive(Debug)]
pub struct Walk {
  pub files: Vec<PathBuf>,
  pub dirs: Vec<PathBuf>,
}

impl Walk {
  pub fn new() -> Self {
    Self {
      files: Vec::with_capacity(1024),
      dirs: Vec::with_capacity(256),
    }
  }

  pub fn walk(&mut self, dir: &PathBuf) -> io::Result<()> {
    for entry in dir.read_dir()? {
      let dir = entry?;
      let path = dir.path();

      if path.is_dir() {
        self.dirs.push(path.to_owned());
        self.walk(&path)?;
      } else {
        self.files.push(path.to_owned());
      }
    }
    io::Result::Ok(())
  }
}
