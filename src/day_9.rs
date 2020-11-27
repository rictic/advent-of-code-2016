#![allow(dead_code)]

use regex::{Match, Regex};

enum DecompressProgress<'a> {
  Rest(&'a str),
  Matched {
    head: &'a str,
    repeat_str: &'a str,
    times: usize,
    tail: &'a str,
  },
}

fn decompress_piece(mut piece: &str) -> DecompressProgress {
  lazy_static! {
    static ref MARKER_RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
  }
  let marker_match: Match = match MARKER_RE.find(piece) {
    Some(m) => m,
    None => {
      return DecompressProgress::Rest(piece);
    }
  };
  let head = &piece[0..marker_match.start()];
  let captures = MARKER_RE.captures(&piece[marker_match.start()..]).unwrap();
  let bytes_to_repeat: usize = captures[1].parse().unwrap();
  let times: usize = captures[2].parse().unwrap();
  piece = &piece[marker_match.end()..];
  let repeat_str = &piece[0..bytes_to_repeat];
  let tail = &piece[bytes_to_repeat..];
  DecompressProgress::Matched {
    head,
    tail,
    repeat_str,
    times,
  }
}

fn decompress(code: &str) -> String {
  let code = code
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
  let mut code = &code[0..];
  let mut result = String::with_capacity(code.len());
  loop {
    match decompress_piece(code) {
      DecompressProgress::Rest(s) => {
        result.push_str(s);
        return result;
      }
      DecompressProgress::Matched {
        head,
        repeat_str,
        times,
        tail,
      } => {
        result.push_str(head);
        for _ in 0..times {
          result.push_str(repeat_str);
        }
        code = tail;
      }
    }
  }
}

fn measure_decompress_v2(code: &str) -> usize {
  let code = code
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
  let mut code = &code[0..];
  let mut len = 0;
  loop {
    match decompress_piece(code) {
      DecompressProgress::Rest(s) => {
        len += s.len();
        return len;
      }
      DecompressProgress::Matched {
        head,
        repeat_str,
        times,
        tail,
      } => {
        len += head.len();
        len += times * measure_decompress_v2(repeat_str);
        code = tail;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!("ADVENT", decompress("ADVENT"));
    assert_eq!(6, decompress("ADVENT").len());
    assert_eq!("ADVENT", decompress(" ADVE NT "));
    assert_eq!(6, decompress(" ADVE NT ").len());
    assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
    assert_eq!(7, decompress("A(1x5)BC").len());
    assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
    assert_eq!(9, decompress("(3x3)XYZ").len());
    assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
    assert_eq!(11, decompress("A(2x2)BCD(2x2)EFG").len());
    assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
    assert_eq!(6, decompress("(6x1)(1x3)A").len());
    assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    assert_eq!(18, decompress("X(8x2)(3x3)ABCY").len());
  }

  #[test]
  fn my_input() {
    assert_eq!(112830, decompress(MY_INPUT).len());
  }

  #[test]
  fn part_2_examples() {
    assert_eq!("XYZXYZXYZ".len(), measure_decompress_v2("(3x3)XYZ"));
    assert_eq!(
      "XABCABCABCABCABCABCY".len(),
      measure_decompress_v2("X(8x2)(3x3)ABCY")
    );
    assert_eq!(
      241920,
      measure_decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A")
    );
    assert_eq!(
      445,
      measure_decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
    );
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(10931789799, measure_decompress_v2(MY_INPUT));
  }

  static MY_INPUT: &'static str = include_str!("day_9_input.txt");
}
