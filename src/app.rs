use std::num::ParseIntError;
use structopt::StructOpt;
use std::path::PathBuf;

fn parse(val: &str) -> Result<u32, ParseIntError> {
  u32::from_str_radix(val, 8)
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "chmods",
  about = "Have separate permissions set recursively for both files and directories"
)]
pub struct App {
  pub entry_path: PathBuf,

  #[structopt(
    short = "d",
    long = "dir-mode",
    default_value = "755",
    parse(try_from_str = parse)
  )]
  pub dir_mode: u32,

  #[structopt(
    short = "f",
    long = "file-mode",
    default_value = "644",
    parse(try_from_str = parse)
  )]
  pub file_mode: u32,
}
