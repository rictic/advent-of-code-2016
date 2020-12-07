#![allow(dead_code)]

use crate::md5::Md5Iterator;
use md5::Digest;
use rayon::prelude::*;
use smallvec::SmallVec;

struct DigestChunks {
  idx: usize,
  chunk_size: usize,
}
impl DigestChunks {
  fn new() -> Self {
    Self {
      idx: 0,
      chunk_size: 1024 * 1024,
    }
  }

  fn next_chunk<'a>(
    &mut self,
    door_id: &'a str,
  ) -> impl rayon::iter::ParallelIterator<Item = Digest> + 'a {
    let range = self.idx..self.idx + self.chunk_size;
    self.idx += self.chunk_size;
    range
      .into_par_iter()
      .map(move |idx| md5::compute(format!("{}{}", door_id, idx)))
      .filter(|digest| digest[0..2] == [0, 0] && digest[2] & 0xf0 == 0)
  }
}

fn get_matched_par<'a>(door_id: &'a str) -> impl rayon::iter::ParallelIterator<Item = Digest> + 'a {
  (0..usize::MAX)
    .into_par_iter()
    .map(move |idx| md5::compute(format!("{}{}", door_id, idx)))
    .filter(|digest| digest[0..2] == [0, 0] && digest[2] & 0xf0 == 0)
}

fn get_matched(door_id: &str) -> impl Iterator<Item = md5::Digest> {
  Md5Iterator::new(door_id).filter(|digest| digest[0..2] == [0, 0] && digest[2] & 0xf0 == 0)
}

fn hex(u4: u8) -> char {
  match u4 {
    0 => '0',
    1 => '1',
    2 => '2',
    3 => '3',
    4 => '4',
    5 => '5',
    6 => '6',
    7 => '7',
    8 => '8',
    9 => '9',
    10 => 'a',
    11 => 'b',
    12 => 'c',
    13 => 'd',
    14 => 'e',
    15 => 'f',
    _ => panic!("Got unexpected value in hex(): {:?}", u4),
  }
}

fn compute_password(door_id: &str) -> String {
  let mut chunks = DigestChunks::new();
  let mut result = String::with_capacity(8);
  while result.len() < 8 {
    result.push_str(
      &chunks
        .next_chunk(door_id)
        .map(|digest| hex(digest[2] & 0x0f))
        .collect::<String>(),
    )
  }
  result.truncate(8);
  result
}

struct CustomSmallVec<A: smallvec::Array> {
  inner: SmallVec<A>,
}
impl<A: smallvec::Array> FromParallelIterator<A::Item> for CustomSmallVec<A>
where
  A::Item: Send,
{
  fn from_par_iter<I>(par_iter: I) -> Self
  where
    I: IntoParallelIterator<Item = A::Item>,
  {
    Self {
      inner: par_iter
        .into_par_iter()
        .fold(
          || SmallVec::new(),
          |mut vec, c| {
            vec.push(c);
            vec
          },
        )
        .reduce(
          || SmallVec::new(),
          |mut l, mut r| {
            l.append(&mut r);
            l
          },
        ),
    }
  }
}

fn compute_complex_password(door_id: &str) -> String {
  let mut result = [None; 8];
  let result_len = result.len();
  let mut chunks = DigestChunks::new();
  while result.iter().any(|v| v.is_none()) {
    let CustomSmallVec {
      inner: postition_chars,
    }: CustomSmallVec<[(usize, char); 3]> = chunks
      .next_chunk(door_id)
      .map(|digest| {
        (
          (digest[2] & 0x0f) as usize,
          format!("{:x}", (digest[3] & 0xf0)).chars().next().unwrap(),
        )
      })
      .filter(|(position, _char)| *position < result_len)
      .collect();
    println!(
      "found {} position_chars in chunk of size {}",
      postition_chars.len(),
      chunks.chunk_size
    );
    for (position, s) in postition_chars {
      if let Some(_) = result[position] {
        continue;
      }
      result[position] = Some(s);
    }
  }
  result.iter().map(|v| v.unwrap()).collect()
}

#[cfg(test)]
mod test {
  #[cfg(not(debug_assertions))]
  use super::*;

  #[cfg(not(debug_assertions))]
  #[test]
  fn example() {
    assert_eq!("18f47a30", &compute_password("abc"));
  }

  const MY_INPUT: &'static str = "reyedfim";

  #[cfg(not(debug_assertions))]
  #[test]
  fn my_input() {
    assert_eq!("f97c354d", &compute_password(MY_INPUT));
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_example() {
    assert_eq!("05ace8e3", &compute_complex_password("abc"))
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_my_input() {
    assert_eq!("863dde27", &compute_complex_password(MY_INPUT))
  }
}
