use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::vec::Vec;
use std::io;
use std::env;
use std::collections::BTreeMap;

struct Tape {
  pos: usize,
  tape: Vec<isize>
}

impl Tape {
  fn new() -> Tape { Tape { pos: 0, tape: vec![0] } }
  fn get(&self) -> isize { self.tape[self.pos] }
  fn getc(&self) -> u8 { self.get() as u8 }
  fn inc(&mut self) { self.tape[self.pos] += 1; }
  fn dec(&mut self) { self.tape[self.pos] -= 1; }
  fn advance(&mut self) { self.pos += 1; if self.tape.len() <= self.pos { self.tape.push(0) } }
  fn devance(&mut self) { if self.pos > 0 { self.pos -= 1; } }
}

struct Program {
  code: Vec<u8>,
  bracket_map: BTreeMap<usize, usize>
}

impl Program {
  fn new(content: Vec<u8>) -> Program {
    let mut code = Vec::new();
    let mut bracket_map = BTreeMap::new();
    let mut leftstack = Vec::new();

    for (pc, b) in content.iter().filter(|&&x| x == b'+' || x == b'-' || x == b'.' || x == b','
            || x == b'<' || x == b'>' || x == b'[' || x == b']').map(|&x| x).enumerate() {
      if b == b'[' {
        leftstack.push(pc);
      } else if b == b']' {
        if let Some(left) = leftstack.pop() {
          bracket_map.insert(left, pc);
          bracket_map.insert(pc, left);
        }
      }
      code.push(b);
    }
    Program{ code: code, bracket_map: bracket_map }
  }

  fn run(&self) {
    let mut pc: usize = 0;
    let mut tape = Tape::new();
    let mut stdout = io::stdout();

    while pc < self.code.len() {
      match self.code[pc] {
        b'+' => tape.inc(),
        b'-' => tape.dec(),
        b'>' => tape.advance(),
        b'<' => tape.devance(),
        b'[' => { if tape.get() == 0 { pc = self.bracket_map[&pc]; } },
        b']' => { if tape.get() != 0 { pc = self.bracket_map[&pc]; } },
        b'.' => { stdout.write(&[tape.getc()]).unwrap(); stdout.flush().unwrap() },
        _ => unreachable!()
      }
      pc += 1;
    }
  }
}

fn main() {
  let mut buf = Vec::new();
  {
    let arg1 = env::args().nth(1).unwrap();
    let path = Path::new(&arg1);
    let mut file = File::open(&path).unwrap();
    file.read_to_end(&mut buf).unwrap();
  }
  Program::new(buf).run();
}
