#![allow(dead_code)]

use std::{error::Error, str::FromStr};

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Register {
  A,
  B,
  C,
  D,
}
impl FromStr for Register {
  type Err = Box<dyn Error>;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let result = match line {
      "a" => Register::A,
      "b" => Register::B,
      "c" => Register::C,
      "d" => Register::D,
      v => return Err(format!("Unknown register: {}", v).into()),
    };
    Ok(result)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegisterOrInteger {
  I(i64),
  R(Register),
}
impl FromStr for RegisterOrInteger {
  type Err = Box<dyn Error>;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let result = match line {
      "a" | "b" | "c" | "d" => RegisterOrInteger::R(line.parse()?),
      v => RegisterOrInteger::I(v.parse()?),
    };
    Ok(result)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
  Copy {
    from: RegisterOrInteger,
    to: Register,
  },
  Increment(Register),
  Decrement(Register),
  JumpNotZero {
    test: RegisterOrInteger,
    offset: RegisterOrInteger,
  },
}
impl FromStr for Instruction {
  type Err = Box<dyn Error>;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref CPY_ARGS_RE: Regex = Regex::new(r"(a|b|c|d|-?\d+) (a|b|c|d)").unwrap();
      static ref JNZ_ARGS_RE: Regex = Regex::new(r"(a|b|c|d|-?\d+) (a|b|c|d|-?\d+)").unwrap();
    }
    let instr = &line[0..3];
    let args = &line[4..];
    let result = match instr {
      "cpy" => {
        let captures = CPY_ARGS_RE
          .captures(args)
          .ok_or("Couldn't parse cpy args")?;

        Instruction::Copy {
          from: captures[1].parse()?,
          to: captures[2].parse()?,
        }
      }
      "inc" => Instruction::Increment(args.parse()?),
      "dec" => Instruction::Decrement(args.parse()?),
      "jnz" => {
        let captures = JNZ_ARGS_RE
          .captures(args)
          .ok_or("Couldn't parse jnz args")?;

        Instruction::JumpNotZero {
          test: captures[1].parse()?,
          offset: captures[2].parse()?,
        }
      }
      s => {
        panic!("Unknown instruction: {}", s)
      }
    };
    Ok(result)
  }
}

struct Computer {
  instructions: Vec<Instruction>,
  program_counter: usize,
  a: i64,
  b: i64,
  c: i64,
  d: i64,
}
impl Computer {
  fn new(program: &str) -> Self {
    Self {
      instructions: program.trim().lines().map(|l| l.parse().unwrap()).collect(),
      program_counter: 0,
      a: 0,
      b: 0,
      c: 0,
      d: 0,
    }
  }

  fn step(&mut self) -> bool {
    let instruction = match self.instructions.get(self.program_counter) {
      Some(i) => *i,
      None => return false,
    };
    match instruction {
      Instruction::Copy { from, to } => {
        self.write_register(to, self.read_register_or_value(from));
      }
      Instruction::Increment(r) => {
        self.write_register(r, self.read_register(r) + 1);
      }
      Instruction::Decrement(r) => {
        self.write_register(r, self.read_register(r) - 1);
      }
      Instruction::JumpNotZero { test, offset } => {
        if self.read_register_or_value(test) != 0 {
          let pc = (self.program_counter as i64)
            .overflowing_add(self.read_register_or_value(offset))
            .0;
          self.program_counter = pc as usize;
          return true;
        }
      }
    }
    self.program_counter += 1;
    return true;
  }

  fn run_to_completion(&mut self) {
    while self.step() {}
  }

  fn read_register_or_value(&self, v: RegisterOrInteger) -> i64 {
    match v {
      RegisterOrInteger::I(i) => i,
      RegisterOrInteger::R(r) => self.read_register(r),
    }
  }

  fn read_register(&self, register: Register) -> i64 {
    match register {
      Register::A => self.a,
      Register::B => self.b,
      Register::C => self.c,
      Register::D => self.d,
    }
  }

  fn write_register(&mut self, register: Register, value: i64) {
    match register {
      Register::A => self.a = value,
      Register::B => self.b = value,
      Register::C => self.c = value,
      Register::D => self.d = value,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let mut computer = Computer::new(EXAMPLE);
    computer.run_to_completion();
    assert_eq!(42, computer.a);
  }

  #[test]
  fn my_input() {
    let mut computer = Computer::new(MY_INPUT);
    computer.run_to_completion();
    assert_eq!(318020, computer.a);
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_my_input() {
    let mut computer = Computer::new(MY_INPUT);
    computer.c = 1;
    computer.run_to_completion();
    assert_eq!(9227674, computer.a);
  }

  static EXAMPLE: &'static str = "\
    cpy 41 a\n\
    inc a\n\
    inc a\n\
    dec a\n\
    jnz a 2\n\
    dec a\n\
  ";
  static MY_INPUT: &'static str = include_str!("day_12_input.txt");
}
