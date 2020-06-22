use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "resetmod",
  about = "Small utility for resetting the fs mode of files and folders"
)]
pub struct App {
  pub path: PathBuf,
}
