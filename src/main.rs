use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

mod binpeeks;
mod peeker;
mod termui;

use crate::binpeeks::BinPeeks;

#[derive(StructOpt)]
struct Opt {
  #[structopt(parse(from_os_str))]
  input: PathBuf,
}

fn main() {
  let opt = Opt::from_args();
  match File::open(&opt.input) {
    Ok(file) => BinPeeks::peek_into(file, opt.input.file_name().unwrap().to_str().unwrap()),
    Err(error) => println!("Error reading {:?}: {}", opt.input, error),
  }
}
