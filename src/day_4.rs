#![allow(dead_code)]
use std::collections::BTreeMap;

use regex::Regex;

struct RoomCode {
  encrypted_name: String,
  sector_id: u64,
  checksum: String,
}
impl RoomCode {
  fn new(raw_code: &str) -> Self {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();
    }
    let cap = RE.captures(raw_code).unwrap();
    let name = cap[1].to_string();
    let sector_id = cap[2].parse().unwrap();
    let checksum = cap[3].to_string();
    RoomCode {
      encrypted_name: name,
      checksum,
      sector_id,
    }
  }

  fn letter_counts(&self) -> BTreeMap<char, i64> {
    let mut result = BTreeMap::new();
    for char in self.encrypted_name.chars().filter(|c| c != &'-') {
      let val = result.entry(char).or_default();
      *val += 1;
    }
    result
  }

  fn compute_checksum(&self) -> String {
    let mut count_letter_pairs = self
      .letter_counts()
      .into_iter()
      .map(|(char, count)| (-count, char))
      .collect::<Vec<_>>();
    count_letter_pairs.sort();
    count_letter_pairs
      .into_iter()
      .take(5)
      .map(|(_count, char)| char)
      .collect()
  }

  fn is_real(&self) -> bool {
    self.checksum == self.compute_checksum()
  }

  fn real_name(&self) -> String {
    let mut result = String::with_capacity(self.encrypted_name.len());
    for ch in self.encrypted_name.as_bytes().iter() {
      if ch == &b'-' {
        result.push(' ');
        continue;
      }
      let mut ch = *ch as u64;
      ch -= b'a' as u64;
      ch += self.sector_id;
      ch %= 26;
      ch += b'a' as u64;
      result.push(ch as u8 as char);
    }
    result
  }
}

fn is_room_real(room: &str) -> bool {
  let room = RoomCode::new(room);
  room.is_real()
}

fn sum_valid_sectors(data: &str) -> u64 {
  data
    .trim_end()
    .lines()
    .map(RoomCode::new)
    .filter(|r| r.is_real())
    .map(|r| r.sector_id)
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(is_room_real("aaaaa-bbb-z-y-x-123[abxyz]"), true);
    assert_eq!(is_room_real("a-b-c-d-e-f-g-h-987[abcde]"), true);
    assert_eq!(is_room_real("not-a-real-room-404[oarel]"), true);
    assert_eq!(is_room_real("totally-real-room-200[decoy]"), false);
  }

  #[test]
  fn example_1() {
    assert_eq!(
      1514,
      sum_valid_sectors(
        "aaaaa-bbb-z-y-x-123[abxyz]\n\
         a-b-c-d-e-f-g-h-987[abcde]\n\
         not-a-real-room-404[oarel]\n\
         totally-real-room-200[decoy]"
      )
    )
  }

  static MY_INPUT: &'static str = include_str!("day_4_input.txt");
  #[test]
  fn my_input() {
    assert_eq!(137896, sum_valid_sectors(MY_INPUT))
  }

  #[test]
  fn part_2_example_1() {
    let room = RoomCode::new("qzmt-zixmtkozy-ivhz-343[abcde]");
    assert_eq!("very encrypted name", room.real_name());
  }

  #[test]
  fn part_2_my_input() {
    let real_rooms: Vec<_> = MY_INPUT
      .trim_end()
      .lines()
      .map(RoomCode::new)
      .filter(|r| r.is_real())
      .filter(|r| {
        let real_name = r.real_name();
        lazy_static! {
          static ref RE: Regex =
            Regex::new(r"rabbit|basket|chocolate|candy|dye|jellybean|bunny|egg|scavenger hunt|flower|plastic grass").unwrap();
        }
        !RE.is_match(&real_name)
      }).collect();
    assert_eq!(real_rooms.len(), 1);
    let room = &real_rooms[0];
    assert_eq!(room.real_name(), "northpole object storage");
    assert_eq!(501, room.sector_id);
  }
}
