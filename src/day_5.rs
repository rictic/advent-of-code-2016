#![allow(dead_code)]

use crate::md5::Md5Iterator;

struct Matcher {
  iter: Md5Iterator,
}
impl Matcher {
  fn new(door_id: &str) -> Self {
    Self {
      iter: Md5Iterator::new(door_id),
    }
  }
}
impl Iterator for Matcher {
  type Item = md5::Digest;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(digest) = self.iter.next() {
      if digest[0..2] == [0, 0] && digest[2] & 0xf0 == 0 {
        return Some(digest);
      }
    }
    None
  }
}

fn compute_password(door_id: &str) -> String {
  let mut result = String::with_capacity(8);
  for digest in Matcher::new(door_id) {
    let s = format!("{:x}", &(digest[2] & 0x0f));
    result.push_str(&s);
    if result.len() >= 8 {
      break;
    }
  }
  result
}

fn compute_complex_password(door_id: &str) -> String {
  let mut result = [None; 8];
  for digest in Matcher::new(door_id) {
    let position = (digest[2] & 0x0f) as usize;
    if position >= result.len() {
      continue;
    }
    if let Some(_) = result[position] {
      continue;
    }
    let s = format!("{:x}", (digest[3] & 0xf0));
    result[position] = s.chars().next();
    if result.iter().all(|v| v.is_some()) {
      return result.iter().map(|v| v.unwrap()).collect();
    }
  }
  panic!("Matcher should iterate forever...");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example() {
    assert_eq!("18f47a30", &compute_password("abc"));
  }

  const MY_INPUT: &'static str = "reyedfim";

  #[test]
  fn my_input() {
    assert_eq!("f97c354d", &compute_password(MY_INPUT));
  }

  #[test]
  fn part_2_example() {
    assert_eq!("05ace8e3", &compute_complex_password("abc"))
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!("863dde27", &compute_complex_password(MY_INPUT))
  }
}
