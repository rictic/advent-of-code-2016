#![allow(dead_code)]

use std::{fmt::Display, str::from_utf8};

fn contains_abba(mut bytes: &[u8]) -> bool {
  println!("Without brackets: {}", from_utf8(bytes).unwrap());
  while bytes.len() >= 4 {
    let ad = bytes;
    bytes = &bytes[1..];
    if ad[0] != ad[3] {
      continue;
    }
    if ad[1] != ad[2] {
      continue;
    }
    if ad[0] == ad[1] {
      continue;
    }
    return true;
  }
  false
}

#[derive(Default)]
struct AbbaMatcher {
  len: usize,
  offset: usize,
  bytes: [u8; 4],
}
impl Display for AbbaMatcher {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.len < self.bytes.len() {
      return f.write_fmt(format_args!(
        "AbbaMatcher {{ {} bytes of {} }}",
        self.len,
        self.bytes.len()
      ));
    }
    let slice = [self.get(0), self.get(1), self.get(2), self.get(3)];
    let str = from_utf8(&slice).unwrap();
    f.write_fmt(format_args!("AbbaMatcher {{ {} }}", str))
  }
}
impl AbbaMatcher {
  fn add_byte(&mut self, byte: u8) {
    if self.len < self.bytes.len() {
      self.len += 1;
    }
    self.bytes[self.offset] = byte;
    self.offset = (self.offset + 1) % self.bytes.len();
  }

  fn is_abba(&self) -> bool {
    if self.len < self.bytes.len() {
      return false;
    }
    if self.get(0) != self.get(3) {
      return false;
    }
    if self.get(1) != self.get(2) {
      return false;
    }
    if self.get(0) == self.get(1) {
      return false;
    }
    return true;
  }

  fn is_aba(&self) -> bool {
    if self.len < 3 {
      return false;
    }
    if self.get(-1) != self.get(-3) {
      return false;
    }
    if self.get(-1) == self.get(-2) {
      return false;
    }
    return true;
  }

  fn get(&self, i: isize) -> u8 {
    let signed_idx: isize = (i + self.offset as isize) % self.bytes.len() as isize;
    let idx = if signed_idx < 0 {
      (self.bytes.len() as isize + signed_idx) as usize
    } else {
      signed_idx as usize
    };
    self.bytes[idx]
  }

  fn clear(&mut self) {
    *self = Self::default()
  }
}

fn supports_tls(address: &str) -> bool {
  let mut matcher = AbbaMatcher::default();
  let mut matched_outside_brackets = false;
  let mut in_brackets = false;
  for byte in address.as_bytes().iter() {
    if *byte == b'[' {
      in_brackets = true;
      matcher.clear();
      continue;
    }
    if in_brackets && *byte == b']' {
      in_brackets = false;
      matcher.clear();
      continue;
    }
    matcher.add_byte(*byte);
    if in_brackets {
      if matcher.is_abba() {
        return false;
      }
    } else {
      if matcher.is_abba() {
        matched_outside_brackets = true;
      }
    }
  }
  matched_outside_brackets
}

fn supports_ssl(address: &str) -> bool {
  let mut matcher = AbbaMatcher::default();
  let mut abas = Vec::new();
  let mut babs = Vec::new();
  let mut in_brackets = false;
  let address = address.as_bytes();
  for (i, byte) in address.iter().enumerate() {
    if *byte == b'[' {
      in_brackets = true;
      matcher.clear();
      continue;
    }
    if in_brackets && *byte == b']' {
      in_brackets = false;
      matcher.clear();
      continue;
    }
    matcher.add_byte(*byte);
    if in_brackets {
      if matcher.is_aba() {
        babs.push(&address[i - 2..=i]);
      }
    } else {
      if matcher.is_aba() {
        abas.push(&address[i - 2..=i]);
      }
    }
  }
  for aba in abas {
    for bab in babs.iter() {
      if aba[0] == bab[1] && aba[1] == bab[0] {
        return true;
      }
    }
  }
  false
}

fn count_support_tls(addresses: &str) -> usize {
  addresses
    .trim_end()
    .lines()
    .filter(|s| supports_tls(*s))
    .count()
}

fn count_support_ssl(addresses: &str) -> usize {
  addresses
    .trim_end()
    .lines()
    .filter(|s| supports_ssl(*s))
    .count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(true, supports_tls("abba[mnop]qrst"));
    assert_eq!(false, supports_tls("abcd[bddb]xyyx"));
    assert_eq!(false, supports_tls("aaaa[qwer]tyui"));
    assert_eq!(true, supports_tls("ioxxoj[asdfgh]zxcvbn"));
  }

  #[test]
  fn example() {
    assert_eq!(2, count_support_tls(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(105, count_support_tls(MY_INPUT));
  }

  #[test]
  fn part_2_examples() {
    assert_eq!(true, supports_ssl("aba[bab]xyz"));
    assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
    assert_eq!(true, supports_ssl("aaa[kek]eke"));
    assert_eq!(true, supports_ssl("zazbz[bzb]cdb"));
  }

  #[test]
  fn part_2_example() {
    assert_eq!(
      3,
      count_support_ssl(
        "aba[bab]xyz\n\
         xyx[xyx]xyx\n\
         aaa[kek]eke\n\
         zazbz[bzb]cdb"
      )
    );
  }
  #[test]
  fn part_2_my_input() {
    assert_eq!(258, count_support_ssl(MY_INPUT));
  }

  const EXAMPLE: &'static str = "abba[mnop]qrst\n\
                                 abcd[bddb]xyyx\n\
                                 aaaa[qwer]tyui\n\
                                 ioxxoj[asdfgh]zxcvbn\n";
  static MY_INPUT: &'static str = include_str!("day_7_input.txt");
}
