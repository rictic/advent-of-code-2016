#![allow(dead_code)]

struct DragonGenerator {
  vec: Vec<bool>,
}
impl DragonGenerator {
  fn new(init: &str) -> DragonGenerator {
    Self {
      vec: init
        .chars()
        .map(|c| match c {
          '0' => false,
          '1' => true,
          _ => panic!("hm"),
        })
        .collect(),
    }
  }
  fn extend(&mut self) {
    let mut tail = self.vec.iter().rev().map(|b| !b).collect::<Vec<_>>();
    self.vec.push(false);
    self.vec.append(&mut tail);
  }
  fn extend_up_to(&mut self, desired_len: usize) {
    while self.vec.len() < desired_len {
      self.extend();
    }
    self.vec.truncate(desired_len);
  }
  fn checksum(&mut self) {
    self.checksum_round();
    while self.vec.len() % 2 == 0 {
      self.checksum_round();
    }
  }
  fn checksum_round(&mut self) {
    let new_len = self.vec.len() / 2;
    for i in 0..new_len {
      self.vec[i] = match (self.vec[i * 2], self.vec[(i * 2) + 1]) {
        (false, false) | (true, true) => true,
        _ => false,
      };
    }
    self.vec.truncate(new_len);
  }
}
impl std::fmt::Display for DragonGenerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for b in self.vec.iter() {
      if *b {
        f.write_str("1")?;
      } else {
        f.write_str("0")?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let mut gen = DragonGenerator::new("1");
    gen.extend();
    assert_eq!("100", format!("{}", gen));
    let mut gen = DragonGenerator::new("0");
    gen.extend();
    assert_eq!("001", format!("{}", gen));
    let mut gen = DragonGenerator::new("11111");
    gen.extend();
    assert_eq!("11111000000", format!("{}", gen));
    let mut gen = DragonGenerator::new("111100001010");
    gen.extend();
    assert_eq!("1111000010100101011110000", format!("{}", gen));

    let mut gen = DragonGenerator::new("10000");
    gen.extend_up_to(20);
    assert_eq!("10000011110010000111", format!("{}", gen));
    gen.checksum();
    assert_eq!("01100", format!("{}", gen));
  }

  #[test]
  fn my_input() {
    let mut gen = DragonGenerator::new("01111010110010011");
    gen.extend_up_to(272);
    gen.checksum();
    assert_eq!("00100111000101111", format!("{}", gen));
  }

  #[test]
  fn part_2_my_input() {
    let mut gen = DragonGenerator::new("01111010110010011");
    gen.extend_up_to(35651584);
    gen.checksum();
    assert_eq!("11101110011100110", format!("{}", gen));
  }
}
