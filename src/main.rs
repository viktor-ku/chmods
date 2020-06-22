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

fn main() -> io::Result<()> {
  let app = App::from_args();
  let starting_path = app.entry_path.as_path().canonicalize()?;

  let mut walk = Walk::new();
  walk.walk(&starting_path)?;

  let file = File::open(starting_path)?;
  file.set_permissions(Permissions::from_mode(app.dir_mode))?;

  for path in walk.dirs {
    let file = File::open(path)?;
    file.set_permissions(Permissions::from_mode(app.dir_mode))?;
  }

  for path in walk.files {
    let file = File::open(path)?;
    file.set_permissions(Permissions::from_mode(app.file_mode))?;
  }

  io::Result::Ok(())
}
