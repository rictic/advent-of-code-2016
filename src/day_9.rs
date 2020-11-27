#![allow(dead_code)]

use std::str::from_utf8;

use regex::bytes::{Match, Regex};

fn decompress(code: &str) -> String {
  let code = code
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
  let mut code = code.as_bytes();
  lazy_static! {
    static ref MARKER_RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
  }
  let mut result = String::with_capacity(code.len());
  loop {
    let marker_match: Match = match MARKER_RE.find(code) {
      Some(m) => m,
      None => {
        result.push_str(from_utf8(code).unwrap());
        return result;
      }
    };
    result.push_str(from_utf8(&code[0..marker_match.start()]).unwrap());
    let captures = MARKER_RE.captures(&code[marker_match.start()..]).unwrap();
    let bytes_to_repeat: usize = from_utf8(&captures[1]).unwrap().parse().unwrap();
    let num_repetitions: usize = from_utf8(&captures[2]).unwrap().parse().unwrap();
    code = &code[marker_match.end()..];
    let repeated_str = from_utf8(&code[0..bytes_to_repeat]).unwrap();
    code = &code[bytes_to_repeat..];
    for _ in 0..num_repetitions {
      result.push_str(repeated_str);
    }
  }
}

fn decompress_v2(code: &str) -> String {
  let result = String::with_capacity(code.len());
  result
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

  static MY_INPUT: &'static str = include_str!("day_9_input.txt");
}
