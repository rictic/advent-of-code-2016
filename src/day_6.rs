#![allow(dead_code)]

use std::collections::BTreeMap;

fn count_chars(repetitions: &str) -> Vec<BTreeMap<char, usize>> {
  let message_len = repetitions.lines().next().unwrap().len();
  let mut character_counts: Vec<BTreeMap<char, usize>> = vec![BTreeMap::new(); message_len];
  for scrambled_message in repetitions.trim_end().lines() {
    for (char, counter) in scrambled_message.chars().zip(character_counts.iter_mut()) {
      let entry = counter.entry(char).or_default();
      *entry += 1;
    }
  }
  character_counts
}

fn decode(repetitions: &str) -> String {
  let character_counts = count_chars(repetitions);
  character_counts
    .iter()
    .map(|counter| {
      counter
        .iter()
        .max_by_key(|(_, v)| **v)
        .map(|(c, _)| *c)
        .unwrap()
    })
    .collect()
}

fn tricky_decode(repetitions: &str) -> String {
  let character_counts = count_chars(repetitions);
  character_counts
    .iter()
    .map(|counter| {
      counter
        .iter()
        .min_by_key(|(_, v)| **v)
        .map(|(c, _)| *c)
        .unwrap()
    })
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example() {
    assert_eq!("easter", decode(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!("umejzgdw", decode(MY_INPUT));
  }

  #[test]
  fn part_2_example() {
    assert_eq!("advent", tricky_decode(EXAMPLE));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!("aovueakv", tricky_decode(MY_INPUT));
  }

  const EXAMPLE: &'static str = "eedadn\n\
                                 drvtee\n\
                                 eandsr\n\
                                 raavrd\n\
                                 atevrs\n\
                                 tsrnev\n\
                                 sdttsa\n\
                                 rasrtv\n\
                                 nssdts\n\
                                 ntnada\n\
                                 svetve\n\
                                 tesnvt\n\
                                 vntsnd\n\
                                 vrdear\n\
                                 dvrsen\n\
                                 enarar";
  static MY_INPUT: &'static str = include_str!("day_6_input.txt");
}
