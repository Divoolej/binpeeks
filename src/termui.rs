use std::sync::mpsc::Receiver;
use std::io::{stdout, Stdout, Write};

use termion::cursor::{Up, Down, Left, Goto};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

pub struct TermUI {
  screen: AlternateScreen<RawTerminal<Stdout>>,
  width: usize,
  height: usize,
  file_name: String,
}

impl TermUI {
  pub fn run(file_name: String, rx: Receiver<String>) {
    std::thread::spawn(move || {
      let mut ui = TermUI::new(file_name);
      ui.init();
      for message in rx {
        ui.write(message);
      }
    });
  }

  fn new(file_name: String) -> TermUI {
    let (width, height) = termion::terminal_size().unwrap();
    TermUI {
      screen: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
      width: width as usize,
      height: height as usize,
      file_name,
    }
  }

  fn draw_borders(&mut self) {
    let horizontal = "═".repeat(self.width - 2);
    let left_down = format!("{}{}", Left(1), Down(1));
    let vertical = format!("║{}", left_down).repeat(self.height);
    // Draw side borders
    write!(self.screen, "{}{}", Goto(1, 1), vertical).unwrap();
    write!(self.screen, "{}{}", Goto(self.width as u16, 1), vertical).unwrap();
    // Draw horizontal borders
    write!(self.screen, "{}╔{}╗", Goto(1, 1), horizontal).unwrap();
    write!(self.screen, "{}╠{}╣", Goto(1, 3), horizontal).unwrap();
    write!(self.screen, "{}╠{}╣", Goto(1, self.height as u16 - 6), horizontal).unwrap();
    write!(self.screen, "{}╠{}╣", Goto(1, self.height as u16 - 2), horizontal).unwrap();
    write!(self.screen, "{}╚{}╝", Goto(1, self.height as u16), horizontal).unwrap();
  }

  fn draw_footer(&mut self) {
    let separator = format!("{}╦{}{}║{}{}╩{} ", Up(1), Left(1), Down(1), Left(1), Down(1), Up(1));
    write!(self.screen, "{}", Goto(3, self.height as u16 - 1)).unwrap();
    write!(self.screen, "h - help {}", separator).unwrap();
    write!(self.screen, "q - quit {}", separator).unwrap();
  }

  fn init(&mut self) {
    let mut file_name = self.file_name.clone();
    if file_name.len() > self.width - 7 {
      file_name.replace_range(
        (self.width / 2 - 5)..(file_name.len() - self.width / 2 + 5), // 5 is just a random offset value
        "..."
      );
    }
    write!(self.screen, "{}", termion::screen::ToAlternateScreen).unwrap();
    write!(self.screen, "{}", termion::clear::All).unwrap();
    write!(self.screen, "{}", termion::cursor::Hide).unwrap();
    self.draw_borders();
    write!(self.screen, "{}> {}", Goto(3, 2), file_name).unwrap();
    self.draw_footer();
    self.screen.flush().unwrap();
  }

  fn write(&mut self, message: String) {
    write!(self.screen, "{}", Goto(3, 5)).unwrap();
    write!(self.screen, "Got: {}", message).unwrap();
    self.screen.flush().unwrap();
  }
}
