use std::io::{stdin, Read};
use std::fs::File;

fn display_help() {
  println!("\n--------Bin Peeks----------");
  println!("The available commands are:");
  println!("next (n) - read the next byte");
  println!("help (h) - display this message");
  println!("exit / quit (q) - exit program");
  println!("---------------------------\n");
}

fn display_bytes(buffer: &[u8]) {
  print!("Binary: ");
  for byte in buffer {
    let mut byte = format!("{:#010b} ", byte);
    byte.replace_range(..2, "");
    print!("{}", byte);
  }
  print!("\nHex: ");
  for byte in buffer { print!("{:02X} ", byte); }
  print!("\nDecimal: ");
  for byte in buffer { print!("{} ", byte); }
  print!("\nASCII: ");
  for byte in buffer {
    let byte = match *byte as char {
      '\n' => "\\n".to_string(),
      '\t' => "\\t".to_string(),
      '\r' => "\\r".to_string(),
      byte => byte.to_string(),
    };
    print!("{}", byte);
  }
  print!("\n");
}

fn parse_number_from_param(param: Option<&str>, default: Option<usize>) -> Option<usize> {
  match param {
    Some(param) => {
      if let Ok(param) = param.to_string().parse::<usize>() {
        Some(param)
      } else {
        None
      }
    },
    None => if let Some(default) = default { Some(default) } else { None },
  }
}

pub struct BinPeeks {
  file: File,
  position: usize,
}

impl BinPeeks {
  pub fn new(file: File) -> BinPeeks {
    BinPeeks {
      file,
      position: 0,
    }
  }

  pub fn peek(&mut self) {
    println!("Ready to read from file, waiting for further commands..");
    let mut input = String::new();
    loop {
      match stdin().read_line(&mut input) {
        Ok(_) => self.parse_command(&input),
        Err(error) => println!("Error: {}", error),
      }
      input.clear();
    }
  }

  pub fn quit(&self) {
    std::process::exit(0);
  }

  fn parse_command(&mut self, command: &String) {
    let mut parts = command.split_whitespace();
    match parts.next() {
      Some("next") | Some("n") => self.read_next_byte(),
      Some("read") | Some("r") => self.read_bytes(parts.next()),
      Some("skip") | Some("s") => self.skip_bytes(parts.next()),
      Some("exit") => self.quit(),
      Some("quit") | Some("q") => self.quit(),
      Some("help") | Some("h") => display_help(),
      Some(cmd) => println!("{} - command not recognized", cmd),
      None => (),
    }
  }

  fn read_next_byte(&mut self) {
    let mut buffer = [0u8];
    self.position += 1;
    match self.file.read_exact(&mut buffer) {
      Ok(_) => {
        println!("Byte {}: ", self.position);
        display_bytes(&buffer);
      }
      Err(_) => println!("Nothing left to read."),
    }
  }

  fn read_bytes(&mut self, bytes_to_read: Option<&str>) {
    if let Some(bytes_to_read) = parse_number_from_param(bytes_to_read, Some(1)) {
      let buffer = self.read_available_bytes(bytes_to_read);
      if buffer.len() > 0 {
        println!("Bytes {} to {}:", self.position + 1, self.position + buffer.len());
        display_bytes(&buffer);
        self.position += buffer.len();
      } else {
        println!("Nothing left to read.");
      }
    } else {
      return println!("Error: incorrect argument to \"read\" - {:?}", bytes_to_read);
    }
  }

  fn skip_bytes(&mut self, bytes_to_read: Option<&str>) {
    if let Some(bytes_to_read) = parse_number_from_param(bytes_to_read, Some(1)) {
      let buffer = self.read_available_bytes(bytes_to_read);
      self.position += buffer.len();
      println!("Skiped {} bytes.", buffer.len());
    } else {
      return println!("Error: incorrect argument to \"read\" - {:?}", bytes_to_read);
    }
  }

  fn read_available_bytes(&mut self, bytes_to_read: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; bytes_to_read];
    if let Err(_) = self.file.read_exact(&mut buffer) {
      // Remove trailing zeroes from buffer
      while let Some(last) = buffer.as_slice().last() {
        if *last == 0 { buffer.pop(); } else { break; }
      }
    }
    buffer
  }
}
