#![allow(dead_code)]

use itertools::Itertools;
use md5::Digest;
use smallvec::SmallVec;
use std::fmt::Write;
use std::{collections::VecDeque, iter::Enumerate};

use crate::md5::Md5Iterator;

struct OneTimePadMatcher<T>
where
  T: Iterator<Item = Digest>,
{
  iter: Enumerate<T>,
  current: Option<(usize, Digest)>,
  candidates: VecDeque<Candidate>,
}
impl<T> OneTimePadMatcher<T>
where
  T: Iterator<Item = Digest>,
{
  fn new(iter: T) -> Self {
    Self {
      iter: iter.enumerate(),
      current: None,
      candidates: VecDeque::new(),
    }
  }

  fn internal_next(&mut self) -> (usize, Digest) {
    if let Some(v) = self.current {
      self.current = None;
      return v;
    }
    self.iter.next().unwrap()
  }
}
impl<T> Iterator for OneTimePadMatcher<T>
where
  T: Iterator<Item = Digest>,
{
  type Item = (usize, Digest);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let (idx, digest) = self.internal_next();
      let runs = HexIterator::new(digest)
        .group_by(|k| *k)
        .into_iter()
        .map(|(k, g)| (k, g.count()))
        .filter(|(_, run_length)| *run_length >= 3)
        .collect::<SmallVec<[(u8, usize); 4]>>();
      if runs.is_empty() {
        continue;
      }

      let cutoff = idx.saturating_sub(1000);
      for (char, run_length) in runs.iter() {
        if *run_length >= 5 {
          for candidate in self.candidates.iter_mut() {
            if candidate.idx < cutoff {
              continue;
            }
            if candidate.running_char == *char {
              candidate.longer_match_found = true;
            }
          }
        }
      }
      let first_run_of_three = runs
        .into_iter()
        .filter(|(_c, usize)| *usize >= 3)
        .map(|(c, _usize)| c)
        .next();
      if let Some(run_of_three) = first_run_of_three {
        self.candidates.push_back(Candidate {
          idx,
          digest,
          running_char: run_of_three,
          longer_match_found: false,
        });
      }

      while let Some(candidate) = self.candidates.get(0) {
        if candidate.idx < cutoff {
          let longer_match_found = candidate.longer_match_found;
          let result = (candidate.idx, candidate.digest);
          self.candidates.pop_front();
          if longer_match_found {
            return Some(result);
          }
        } else {
          break;
        }
      }
    }
  }
}

struct HexIterator {
  digest: Digest,
  idx: usize,
  lower: bool,
}
impl HexIterator {
  fn new(digest: Digest) -> Self {
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
struct Candidate {
  idx: usize,
  running_char: u8,
  digest: Digest,
  longer_match_found: bool,
}

struct StretchedHashIter {
  md5_iter: Md5Iterator,
  scratch_string: String,
}

impl StretchedHashIter {
  fn new(seed: &str) -> Self {
    Self {
      md5_iter: Md5Iterator::new(seed),
      scratch_string: String::with_capacity(32),
    }
  }
}

impl Iterator for StretchedHashIter {
  type Item = Digest;

  fn next(&mut self) -> Option<Self::Item> {
    let mut digest = self.md5_iter.next()?;
    for _ in 0..2016 {
      self.scratch_string.clear();
      write!(self.scratch_string, "{:x}", digest).ok()?;
      digest = md5::compute(&self.scratch_string);
    }
    Some(digest)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let digest = md5::compute("abc18");
    let hexes: String = HexIterator::new(digest)
      .map(|u| format!("{:x}", u))
      .collect();
    assert_eq!(hexes, format!("{:x}", digest));
    assert_eq!("0034e0923cc38887a57bd7b1d4f953df", hexes);
    let mut matcher = OneTimePadMatcher::new(Md5Iterator::new("abc"));
    assert_eq!(
      Some((
        39,
        Digest([
          0x34, 0x7d, 0xac, 0x6e, 0xe8, 0xee, 0xea, 0x46, 0x52, 0xc7, 0x47, 0x6d, 0x0f, 0x97, 0xbe,
          0xe5
        ])
      )),
      matcher.next()
    );
    assert_eq!(Some(92), matcher.next().map(|(idx, _digest)| idx));

    let matcher = OneTimePadMatcher::new(Md5Iterator::new("abc"));
    let expected_sixty_fourth_idx = 22728;
    assert_eq!(
      Some(63),
      matcher
        .enumerate()
        .take(500)
        .find(|(_oidx, (idx, _digest))| *idx == expected_sixty_fourth_idx)
        .map(|(oidx, _)| oidx)
    );
    assert_eq!(
      Some(expected_sixty_fourth_idx),
      OneTimePadMatcher::new(Md5Iterator::new("abc"))
        .skip(63)
        .next()
        .map(|(oidx, _)| oidx)
    );
  }

  const MY_INPUT: &'static str = "ihaygndm";
  #[test]
  fn my_input() {
    assert_eq!(
      Some(15035),
      OneTimePadMatcher::new(Md5Iterator::new(MY_INPUT))
        .skip(63)
        .next()
        .map(|(oidx, _)| oidx)
    );
  }

  #[test]
  fn part_2_example() {
    assert_eq!(
      Some(22551),
      OneTimePadMatcher::new(StretchedHashIter::new("abc"))
        .skip(63)
        .next()
        .map(|(oidx, _)| oidx)
    );
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(
      Some(19968),
      OneTimePadMatcher::new(StretchedHashIter::new(MY_INPUT))
        .skip(63)
        .next()
        .map(|(oidx, _)| oidx)
    );
  }
}
