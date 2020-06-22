use structopt::StructOpt;
use std::{
  io,
  os::unix::fs::{
    PermissionsExt,
  },
  fs::{
    File,
    Permissions,
  },
};

mod app;
use app::App;

mod walk;
use walk::Walk;

const DIR_MODE: u32 = 0o755;
const FILE_MODE: u32 = 0o644;

fn main() -> io::Result<()> {
  let app = App::from_args();
  let starting_path = app.path.as_path().canonicalize()?;

  let mut walk = Walk::new();
  walk.walk(&starting_path)?;

  let file = File::open(starting_path)?;
  file.set_permissions(Permissions::from_mode(DIR_MODE))?;

  for path in walk.dirs {
    let file = File::open(path)?;
    file.set_permissions(Permissions::from_mode(DIR_MODE))?;
  }

  for path in walk.files {
    let file = File::open(path)?;
    file.set_permissions(Permissions::from_mode(FILE_MODE))?;
  }

  io::Result::Ok(())
}
