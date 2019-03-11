use std::fs::File;
use std::sync::mpsc;
use std::io::stdin;

use termion::event::Key;
use termion::input::TermRead;

use crate::termui::TermUI;

pub struct BinPeeks {
  file_name: String,
}

impl BinPeeks {
  pub fn peek_into(file: File, file_name: &str) {
    BinPeeks::new(file, file_name).run();
  }

  fn new(_file: File, file_name: &str) -> BinPeeks {
    BinPeeks {
      file_name: file_name.to_string(),
    }
  }

  fn run(&mut self) {
    let (tx, rx) = mpsc::channel();
    TermUI::run(self.file_name.clone(), rx);
    // clone tx and pass it to a thread for signal handlers

    let stdin = stdin();
    for c in stdin.keys() {
      match c.unwrap() {
        Key::Char(c) => tx.send(c.to_string()).unwrap(),
        _ => tx.send("other".to_string()).unwrap(),
      }
    }
  }
}
