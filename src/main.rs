use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

mod binpeek;

use crate::binpeek::BinPeek;

#[derive(StructOpt)]
struct Opt {
  #[structopt(parse(from_os_str))]
  input: PathBuf,
}

fn main() {
  let opt = Opt::from_args();
  println!("Peeking into {:?}...", opt.input);
  match File::open(&opt.input) {
    Ok(file) => BinPeek::new(file).peek(),
    Err(error) => println!("Error reading {:?}: {}", opt.input, error),
  }
}
