#![allow(dead_code)]

use smallvec::SmallVec;
use Direction::*;

use crate::{astar::AStarSearcher, md5::HexIterator};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
impl Direction {
  fn as_char(&self) -> char {
    match self {
      Up => 'U',
      Down => 'D',
      Left => 'L',
      Right => 'R',
    }
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct MoveList {
  moves: SmallVec<[Direction; 16]>,
}

impl MoveList {
  fn with_move(&self, direction: Direction) -> MoveList {
    let mut moves = self.moves.clone();
    moves.push(direction);
    MoveList { moves }
  }

  fn location(&self) -> (i64, i64) {
    self.moves.iter().fold((0, 0), |(x, y), d| match d {
      Up => (x, y - 1),
      Down => (x, y + 1),
      Left => (x - 1, y),
      Right => (x + 1, y),
    })
  }

  fn neighbors(&self, options: SmallVec<[Direction; 4]>) -> Vec<MoveList> {
    let (x, y) = self.location();
    let mut results = Vec::with_capacity(4);
    for direction in options.into_iter() {
      match direction {
        Up => {
          if y > 0 {
            results.push(self.with_move(direction));
          }
        }
        Down => {
          if y < 3 {
            results.push(self.with_move(direction));
          }
        }
        Left => {
          if x > 0 {
            results.push(self.with_move(direction));
          }
        }
        Right => {
          if x < 3 {
            results.push(self.with_move(direction));
          }
        }
      }
    }
    results
  }
}

struct Vault {
  passcode_len: usize,
  scratch_str: String,
}

impl Vault {
  fn new(passcode: &str) -> Self {
    Self {
      passcode_len: passcode.len(),
      scratch_str: passcode.to_string(),
    }
  }

  fn find_doors(&mut self, moves: &MoveList) -> SmallVec<[MoveList; 4]> {
    let (x, y) = moves.location();
    if x == 3 && y == 3 {
      return Default::default();
    }
    self.scratch_str.truncate(self.passcode_len);
    for direction in moves.moves.iter() {
      self.scratch_str.push(direction.as_char());
    }
    let digest = md5::compute(&self.scratch_str);
    let mut hexes = HexIterator::new(digest);
    let mut results = SmallVec::new();

    if hexes.next().unwrap() >= 0xb && y > 0 {
      results.push(moves.with_move(Up));
    }
    if hexes.next().unwrap() >= 0xb && y < 3 {
      results.push(moves.with_move(Down));
    }
    if hexes.next().unwrap() >= 0xb && x > 0 {
      results.push(moves.with_move(Left));
    }
    if hexes.next().unwrap() >= 0xb && x < 3 {
      results.push(moves.with_move(Right));
    }
    results
  }
}

impl AStarSearcher for Vault {
  type Node = MoveList;
  type Successors = SmallVec<[Self::Node; 4]>;

  fn optimistic_distance(&self, node: &Self::Node) -> u64 {
    let (x, y) = node.location();
    ((3 - x).abs() as u64) + ((3 - y).abs() as u64)
  }

  fn successors(&mut self, node: &Self::Node) -> Self::Successors {
    self.find_doors(&node)
  }
}

fn problem(passcode: &str) -> String {
  let mut vault = Vault::new(passcode);
  let (_len, moves) = vault
    .search(MoveList {
      moves: Default::default(),
    })
    .unwrap();
  moves.moves.into_iter().map(|d| d.as_char()).collect()
}

fn part_2_par(passcode: &str, node: &MoveList) -> usize {
  let mut vault = Vault::new(passcode);
  if node.location() == (3, 3) {
    return node.moves.len();
  }
  let mut solution_len: usize = 0;
  let best_child = vault
    .find_doors(node)
    .par_iter()
    .map(|node| part_2_par(passcode, node))
    .reduce(|| 0, |left, right| left.max(right));
  solution_len = solution_len.max(best_child);
  solution_len
}

fn problem_part_2(passcode: &str) -> usize {
  part_2_par(
    passcode,
    &MoveList {
      moves: Default::default(),
    },
  )
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!("DDRRRD", problem("ihgpwlah"));
    assert_eq!("DDUDRLRRUDRD", problem("kglvqrro"));
    assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", problem("ulqzkmiv"));
  }

  #[test]
  fn my_input() {
    assert_eq!("RLRDRDUDDR", problem("rrrbmfta"));
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn examples_part_2() {
    assert_eq!(370, problem_part_2("ihgpwlah"));
    assert_eq!(492, problem_part_2("kglvqrro"));
    assert_eq!(830, problem_part_2("ulqzkmiv"));
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_my_input() {
    assert_eq!(420, problem_part_2("rrrbmfta"));
  }
}
