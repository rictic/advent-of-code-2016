#![allow(dead_code)]

use std::fmt::Write;
pub struct Md5Iterator {
  len: usize,
  i: u64,
  scratch_str: String,
}
impl Md5Iterator {
  pub fn new(prefix: &str) -> Self {
    Self {
      len: prefix.len(),
      i: 0,
      scratch_str: prefix.to_string(),
    }
  }
}
impl Iterator for Md5Iterator {
  type Item = md5::Digest;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      self.scratch_str.truncate(self.len);
      write!(self.scratch_str, "{}", self.i).unwrap();
      let digest = md5::compute(&self.scratch_str);
      self.i += 1;
      return Some(digest);
    }
  }
}
