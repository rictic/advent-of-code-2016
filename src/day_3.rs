#![allow(dead_code)]
type Candidate = (u64, u64, u64);

fn is_valid_triangle(candidate: Candidate) -> bool {
  let (a, b, c) = candidate;
  if a + b <= c {
    return false;
  }
  if a + c <= b {
    return false;
  }
  if b + c <= a {
    return false;
  }
  true
}

fn read_num(input: &[u8]) -> (&[u8], u64) {
  let mut i = 0;
  loop {
    match input[i] {
      b' ' => i += 1,
      _ => break,
    }
  }
  let start = i;
  loop {
    let c = match input.get(i) {
      None => break,
      Some(c) => c,
    };
    match c {
      b'0'..=b'9' => i += 1,
      _ => break,
    }
  }
  let end = i;
  while input.get(i) == Some(&b'\n') {
    i += 1
  }
  let val = std::str::from_utf8(&input[start..end])
    .unwrap()
    .parse()
    .unwrap();
  (&input[i..], val)
}

struct CandidateReader<'a> {
  candidates: &'a [u8],
}
impl<'a> CandidateReader<'a> {
  fn new(candidates: &'a str) -> Self {
    Self {
      candidates: candidates.as_bytes(),
    }
  }
  fn read_num(&mut self) -> u64 {
    let (updated, val) = read_num(self.candidates);
    self.candidates = updated;
    val
  }
}
impl<'a> Iterator for CandidateReader<'a> {
  type Item = Candidate;

  fn next(&mut self) -> Option<Self::Item> {
    if self.candidates.len() == 0 {
      return None;
    }
    Some((self.read_num(), self.read_num(), self.read_num()))
  }
}

struct VerticalCandidateReader<'a> {
  candidates: &'a [u8],
  buf_one: Option<Candidate>,
  buf_two: Option<Candidate>,
}
impl<'a> VerticalCandidateReader<'a> {
  fn new(candidates: &'a str) -> Self {
    Self {
      candidates: candidates.as_bytes(),
      buf_one: None,
      buf_two: None,
    }
  }
  fn read_num(&mut self) -> u64 {
    let (updated, val) = read_num(self.candidates);
    self.candidates = updated;
    val
  }
}
impl<'a> Iterator for VerticalCandidateReader<'a> {
  type Item = Candidate;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(v) = self.buf_one {
      self.buf_one = None;
      return Some(v);
    }
    if let Some(v) = self.buf_two {
      self.buf_two = None;
      return Some(v);
    }
    if self.candidates.len() == 0 {
      return None;
    }
    let (a1, b1, c1) = (self.read_num(), self.read_num(), self.read_num());
    let (a2, b2, c2) = (self.read_num(), self.read_num(), self.read_num());
    let (a3, b3, c3) = (self.read_num(), self.read_num(), self.read_num());
    self.buf_one = Some((b1, b2, b3));
    self.buf_two = Some((c1, c2, c3));
    Some((a1, a2, a3))
  }
}

fn count_valid_triangles(iter: &mut dyn Iterator<Item = Candidate>) -> usize {
  iter.filter(|c| is_valid_triangle(*c)).count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(false, is_valid_triangle((5, 10, 25)));
  }

  #[test]
  fn example_2() {
    assert_eq!(
      1,
      count_valid_triangles(&mut CandidateReader::new("5 10 25\n  3 4 5"))
    )
  }

  static MY_INPUT: &'static str = include_str!("day_3_input.txt");

  #[test]
  fn my_input() {
    assert_eq!(
      982,
      count_valid_triangles(&mut CandidateReader::new(MY_INPUT))
    )
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(
      1826,
      count_valid_triangles(&mut VerticalCandidateReader::new(MY_INPUT))
    )
  }
}
