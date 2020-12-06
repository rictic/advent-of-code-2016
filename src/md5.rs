#![allow(dead_code)]

use std::fmt::Write;

use md5::Digest;
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
  type Item = Digest;

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

pub struct HexIterator {
  digest: Digest,
  idx: usize,
  lower: bool,
}
impl HexIterator {
  pub fn new(digest: Digest) -> Self {
    Self {
      digest,
      idx: 0,
      lower: false,
    }
  }
}
impl Iterator for HexIterator {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    let initial = *self.digest.0.get(self.idx)?;
    let c = if self.lower {
      self.lower = false;
      self.idx += 1;
      initial & 0x0f
    } else {
      self.lower = true;
      (initial & 0xf0) >> 4
    };
    Some(c)
  }
}
