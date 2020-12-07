#![allow(dead_code)]

use crate::astar::AStarSearcher;
use itertools::Itertools;
use smallvec::SmallVec;
use std::fmt::Display;
use MachineKind::*;

struct MachineInit {
  name: char,
  chip_floor: usize,
  generator_floor: usize,
}

#[derive(Default)]
struct State {
  // The names of the different kinds of devices
  names: Vec<char>,
  initial: InnerState,
}
impl State {
  fn new(init: impl Iterator<Item = MachineInit>) -> Self {
    let mut result = Self::default();
    for (i, machine) in init.enumerate() {
      result.names.push(machine.name);
      result.initial.floors[machine.generator_floor].set(Mask::new(i as u8, Generator));
      result.initial.floors[machine.chip_floor].set(Mask::new(i as u8, Chip));
    }
    result
  }

  fn count_moves_to_solution(self) -> Option<u64> {
    return StateSearcher::default()
      .caching()
      .search(self.initial)
      .map(|s| s.0);
  }
}
impl Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    /*
    F4
    F3            LG
    F2    HG
    F1 E     M       M
    */
    for floor in (0..4).into_iter().map(|i| 3 - i) {
      f.write_fmt(format_args!("F{} ", floor + 1))?;
      if self.initial.elevator == floor {
        f.write_str("E")?;
      } else {
        f.write_str(" ")?;
      }
      for (idx, name) in self.names.iter().enumerate() {
        if self.initial.floors[floor as usize].get(Mask::new(idx as u8, Generator)) {
          f.write_fmt(format_args!("   {}G ", name))?;
        } else {
          f.write_str("      ")?;
        }
        if self.initial.floors[floor as usize].get(Mask::new(idx as u8, Chip)) {
          f.write_str("M")?;
        } else {
          f.write_str(" ")?;
        }
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}
#[derive(Default)]
struct StateSearcher {}
impl crate::astar::AStarSearcher for StateSearcher {
  type Node = InnerState;
  type Successors = SmallVec<[Self::Node; 8]>;

  fn optimistic_distance(&self, node: &Self::Node) -> u64 {
    node.distance_from_complete()
  }

  fn successors(&mut self, node: &Self::Node) -> Self::Successors {
    node.successors().collect()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MachineKind {
  Generator,
  Chip,
}

struct Mask(u16);
impl Mask {
  fn new(idx: u8, kind: MachineKind) -> Self {
    let kind_offset = match kind {
      MachineKind::Generator => 0,
      MachineKind::Chip => 1,
    };
    return Mask(1 << ((idx as u16) * 2) + kind_offset);
  }

  fn all() -> impl Iterator {
    (0..14).into_iter().map(|idx| 1 << idx)
  }
}
impl std::ops::Not for Mask {
  type Output = Self;

  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}
// This is a dense, minimal representation of the mutable parts of the world state.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InnerState {
  elevator: u8,
  floors: [Floor; 4],
}
impl InnerState {
  fn is_finished(&self) -> bool {
    self.elevator == 3 && self.floors[0..3] == [Floor(0), Floor(0), Floor(0)]
  }

  fn successors(&self) -> SuccessorsIterator {
    SuccessorsIterator::new(*self)
  }

  fn distance_from_complete(&self) -> u64 {
    ((self.floors[0].len() * 3) + (self.floors[1].len() * 2) + (self.floors[2].len())) as u64
  }
}
struct SuccessorsIterator {
  machine_pairs: Vec<(Machine, Option<Machine>)>,
  floor_idx: usize,
  floors_to_process: (u8, Option<u8>),
  machine_pairs_idx: usize,
  state: InnerState,
}
impl SuccessorsIterator {
  fn new(state: InnerState) -> Self {
    let floor = state.floors[state.elevator as usize];
    let num_machines = floor.len();
    let mut machine_pairs: Vec<(Machine, Option<Machine>)> =
      Vec::with_capacity((num_machines * (num_machines - 1)) as usize);
    for (i, m1) in floor.into_iter().enumerate() {
      for m2 in floor.into_iter().skip(i + 1).into_iter() {
        machine_pairs.push((m1, Some(m2)))
      }
      machine_pairs.push((m1, None));
    }

    let floors_to_process = if state.elevator == 0 {
      (1, None)
    } else if state.elevator == 3 {
      (2, None)
    } else {
      (state.elevator + 1, Some(state.elevator - 1))
    };

    // for (m1, m2) in machine_pairs.iter() {
    Self {
      machine_pairs,
      machine_pairs_idx: 0,
      floor_idx: 0,
      floors_to_process,
      state,
    }
  }
}
impl Iterator for SuccessorsIterator {
  type Item = InnerState;

  fn next(&mut self) -> Option<Self::Item> {
    if self.floor_idx == 2 {
      return None;
    }
    let floor_into = if self.floor_idx == 0 {
      self.floors_to_process.0
    } else {
      match self.floors_to_process.1 {
        Some(floor) => floor,
        None => return None,
      }
    };
    loop {
      let (m1, m2) = match self.machine_pairs.get(self.machine_pairs_idx) {
        Some(v) => {
          self.machine_pairs_idx += 1;
          v
        }
        None => {
          self.machine_pairs_idx = 0;
          self.floor_idx += 1;
          return self.next();
        }
      };
      let mut new_floor_from = self.state.floors[self.state.elevator as usize];
      let mut new_floor_into = self.state.floors[floor_into as usize];
      new_floor_from.unset(m1.mask());
      new_floor_into.set(m1.mask());
      if let Some(m2) = m2 {
        new_floor_from.unset(m2.mask());
        new_floor_into.set(m2.mask());
      }
      if !(new_floor_from.is_valid() && new_floor_into.is_valid()) {
        continue;
      }
      let mut floors = self.state.floors.clone();
      floors[self.state.elevator as usize] = new_floor_from;
      floors[floor_into as usize] = new_floor_into;

      return Some(InnerState {
        elevator: floor_into,
        floors,
      });
    }
  }
}

const ALL_MACHINES: [Machine; 14] = [
  Machine(0, Generator),
  Machine(0, Chip),
  Machine(1, Generator),
  Machine(1, Chip),
  Machine(2, Generator),
  Machine(2, Chip),
  Machine(3, Generator),
  Machine(3, Chip),
  Machine(4, Generator),
  Machine(4, Chip),
  Machine(5, Generator),
  Machine(5, Chip),
  Machine(6, Generator),
  Machine(6, Chip),
];
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Machine(u8, MachineKind);
impl Machine {
  fn mask(&self) -> Mask {
    Mask::new(self.0, self.1)
  }
}
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Floor(u16);
impl Floor {
  fn get(&self, mask: Mask) -> bool {
    (self.0 & mask.0) != 0
  }
  fn set(&mut self, mask: Mask) {
    self.0 |= mask.0;
  }
  fn unset(&mut self, mask: Mask) {
    self.0 &= !mask.0;
  }
  fn len(&self) -> u32 {
    self.0.count_ones()
  }
  fn is_valid(&self) -> bool {
    if self.0 & 0b01010101010101 == 0 {
      // no generator, so no risk
      return true;
    }
    // look for a chip without a protective generator
    let machine_pairs = self.into_iter().group_by(|m| m.0);
    for (_key, mut group) in &machine_pairs {
      let fst = group.next().unwrap();
      let snd = group.next();
      if snd.is_some() {
        // Generator and chip pair, this is fine.
        continue;
      }
      if fst.1 == Chip {
        // Bare chip in danger
        return false;
      }
    }
    return true;
  }
}
impl IntoIterator for Floor {
  type Item = Machine;

  type IntoIter = FloorIter;

  fn into_iter(self) -> Self::IntoIter {
    FloorIter { floor: self, i: 0 }
  }
}
#[derive(Clone, Copy)]
struct FloorIter {
  floor: Floor,
  i: usize,
}
impl Iterator for FloorIter {
  type Item = Machine;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(machine) = ALL_MACHINES.get(self.i) {
      self.i += 1;
      if self.floor.get(machine.mask()) {
        return Some(*machine);
      }
    }
    None
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  /// Wrapper around string slice that makes debug output `{:?}` to print string same way as `{}`.
  /// Used in different `assert*!` macros in combination with `pretty_assertions` crate to make
  /// test failures to show nice diffs.
  #[derive(PartialEq, Eq)]
  #[doc(hidden)]
  pub struct PrettyString(pub String);

  /// Make diff to display string as multi-line string
  impl<'a> std::fmt::Debug for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.write_str(&self.0)
    }
  }

  fn pretty(s: &str) -> PrettyString {
    PrettyString(s.to_string())
  }

  #[test]
  fn examples() {
    /*
      The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
      The second floor contains a hydrogen generator.
      The third floor contains a lithium generator.
      The fourth floor contains nothing relevant.
    */
    let state = State::new(
      vec![
        MachineInit {
          name: 'H',
          chip_floor: 0,
          generator_floor: 1,
        },
        MachineInit {
          name: 'L',
          chip_floor: 0,
          generator_floor: 2,
        },
      ]
      .into_iter(),
    );
    assert_eq!(
      pretty(
        "\
          F4                \n\
          F3            LG  \n\
          F2     HG         \n\
          F1 E      M      M\n\
        "
      ),
      pretty(&format!("{}", state))
    );
    assert_eq!(
      vec![pretty(
        "\
          F4                \n\
          F3            LG  \n\
          F2 E   HG M       \n\
          F1               M\n\
        "
      ),],
      state
        .initial
        .successors()
        .map(|s| {
          let s = State {
            names: state.names.clone(),
            initial: s,
          };
          pretty(&format!("{}", s))
        })
        .collect::<Vec<PrettyString>>()
    );
    assert_eq!(
      vec![
        pretty(
          "\
            F4                \n\
            F3 E   HG M   LG  \n\
            F2                \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3 E   HG     LG  \n\
            F2        M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3            LG  \n\
            F2     HG         \n\
            F1 E      M      M\n\
          "
        ),
      ],
      state
        .initial
        .successors()
        .flat_map(|s| s.successors())
        .map(|s| {
          let s = State {
            names: state.names.clone(),
            initial: s,
          };
          pretty(&format!("{}", s))
        })
        .collect::<Vec<PrettyString>>()
    );
    assert_eq!(
      vec![
        pretty(
          "\
            F4 E   HG M       \n\
            F3            LG  \n\
            F2                \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E   HG     LG  \n\
            F3        M       \n\
            F2                \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E      M       \n\
            F3     HG     LG  \n\
            F2                \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E          LG  \n\
            F3     HG M       \n\
            F2                \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3            LG  \n\
            F2 E   HG M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3        M       \n\
            F2 E   HG     LG  \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3     HG     LG  \n\
            F2 E      M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3     HG M       \n\
            F2 E          LG  \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E   HG     LG  \n\
            F3                \n\
            F2        M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E   HG         \n\
            F3            LG  \n\
            F2        M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4 E          LG  \n\
            F3     HG         \n\
            F2        M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3                \n\
            F2 E   HG M   LG  \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3            LG  \n\
            F2 E   HG M       \n\
            F1               M\n\
          "
        ),
        pretty(
          "\
            F4                \n\
            F3            LG  \n\
            F2 E   HG M       \n\
            F1               M\n\
          "
        ),
      ],
      state
        .initial
        .successors()
        .flat_map(|s| s.successors())
        .flat_map(|s| s.successors())
        .map(|s| {
          let s = State {
            names: state.names.clone(),
            initial: s,
          };
          pretty(&format!("{}", s))
        })
        .collect::<Vec<PrettyString>>()
    );
    assert_eq!(Some(11), state.count_moves_to_solution());
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn my_input() {
    /*
    The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
    The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
    The third floor contains nothing relevant.
    The fourth floor contains nothing relevant.
    */
    let state = State::new(
      vec![
        MachineInit {
          name: 'P',
          generator_floor: 0,
          chip_floor: 1,
        },
        MachineInit {
          name: 'T',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'p',
          generator_floor: 0,
          chip_floor: 1,
        },
        MachineInit {
          name: 'R',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'C',
          generator_floor: 0,
          chip_floor: 0,
        },
      ]
      .into_iter(),
    );
    assert_eq!(Some(47), state.count_moves_to_solution());
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_my_input() {
    let state = State::new(
      vec![
        MachineInit {
          name: 'P',
          generator_floor: 0,
          chip_floor: 1,
        },
        MachineInit {
          name: 'T',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'p',
          generator_floor: 0,
          chip_floor: 1,
        },
        MachineInit {
          name: 'R',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'C',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'E',
          generator_floor: 0,
          chip_floor: 0,
        },
        MachineInit {
          name: 'D',
          generator_floor: 0,
          chip_floor: 0,
        },
      ]
      .into_iter(),
    );
    assert_eq!(Some(71), state.count_moves_to_solution());
  }
}
