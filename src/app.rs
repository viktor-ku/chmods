use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "chmods",
  about = "Have separate permissions set recursively for both files and directories"
)]
pub struct App {
  pub entry_path: PathBuf,
}
