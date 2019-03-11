use std::fs::File;

pub struct Peeker {
  file: File,
  position: usize,
}

impl Peeker {
  pub fn new(file: File) -> Peeker {
    Peeker {
      file,
      position: 0,
    }
  }
}
