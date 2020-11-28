#![allow(dead_code)]
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
  Initialize { value: u8, bot: u8 },
  Rule { bot: u8, rule: Rule },
}
impl Instruction {
  fn parse(line: &str) -> Self {
    lazy_static! {
      static ref INITIALIZE_RE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
      static ref RULE_RE: Regex =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
          .unwrap();
    }
    if let Some(captures) = INITIALIZE_RE.captures(line) {
      let value = captures[1].parse().unwrap();
      let bot = captures[2].parse().unwrap();
      return Instruction::Initialize { value, bot };
    }
    fn parse_destination(bot_or_output: &str, num_str: &str) -> Destination {
      let num = num_str.parse().unwrap();
      if bot_or_output == "bot" {
        return Destination::Bot(num);
      } else {
        return Destination::Output(num);
      }
    }

    if let Some(captures) = RULE_RE.captures(line) {
      let bot = captures[1].parse().unwrap();
      let low = parse_destination(&captures[2], &captures[3]);
      let high = parse_destination(&captures[4], &captures[5]);
      return Instruction::Rule {
        bot,
        rule: Rule { low, high },
      };
    }
    todo!()
  }
}

#[derive(Debug)]
struct Rule {
  low: Destination,
  high: Destination,
}
#[derive(Debug, Clone, Copy)]
enum Destination {
  Bot(u8),
  Output(u8),
}
#[derive(Debug)]
struct Bot {
  left_hand: Option<u8>,
  right_hand: Option<u8>,
  rule: Rule,
}
impl Bot {
  fn take(&mut self, value: u8) -> bool {
    if self.left_hand.is_none() {
      self.left_hand = Some(value);
      return false;
    }
    if self.right_hand.is_none() {
      self.right_hand = Some(value);
      return true;
    }
    panic!("Bot with full hands was handed value {}", value);
  }
}
#[derive(Debug)]
struct State {
  bots: Vec<Bot>,
  outputs: Vec<Vec<u8>>,
}
impl State {
  fn new(instructions: &str) -> (Self, u8) {
    let mut instructions: Vec<_> = instructions
      .lines()
      .map(|line| Instruction::parse(line))
      .collect();
    instructions.sort_by_key(|instruction: &Instruction| match instruction {
      Instruction::Initialize { bot, .. } => (*bot, 1),
      Instruction::Rule { bot, .. } => (*bot, 0),
    });
    let mut bots = Vec::with_capacity(instructions.len() * 2);
    let mut outputs = Vec::new();
    let instructions = instructions
      .into_iter()
      .group_by(|instruction: &Instruction| match instruction {
        Instruction::Initialize { bot, .. } => *bot,
        Instruction::Rule { bot, .. } => *bot,
      });
    let mut full_hands_bot = None;
    for (bot_id, mut instructions) in &instructions {
      assert!(bots.len() == bot_id as usize);

      let rule = instructions.next();
      let left_init = instructions.next();
      let right_init = instructions.next();
      assert!(instructions.next().is_none());
      let rule = match rule {
        Some(Instruction::Rule { bot: _bot, rule }) => rule,
        _ => panic!(
          "Expected to find a Rule for bot {} but found {:?}",
          bot_id, rule
        ),
      };
      let left_hand = if let Some(Instruction::Initialize { bot: _bot, value }) = left_init {
        Some(value)
      } else {
        None
      };
      let right_hand = if let Some(Instruction::Initialize { bot: _bot, value }) = right_init {
        Some(value)
      } else {
        None
      };
      for dest in &[rule.low, rule.high] {
        if let Destination::Output(idx) = dest {
          while outputs.len() <= *idx as usize {
            outputs.push(Vec::new());
          }
        }
        match dest {
          Destination::Output(_) => {}
          Destination::Bot(_) => {}
        }
      }
      if left_hand.is_some() && right_hand.is_some() {
        assert!(full_hands_bot.is_none());
        full_hands_bot = Some(bot_id);
      }
      bots.push(Bot {
        rule,
        left_hand,
        right_hand,
      });
    }
    assert!(full_hands_bot.is_some());
    (Self { bots, outputs }, full_hands_bot.unwrap())
  }

  fn tick(&mut self, full_hands_bot: u8) -> Vec<u8> {
    let bot = &mut self.bots[full_hands_bot as usize];
    let (lesser, greater) = match (bot.left_hand, bot.right_hand) {
      (Some(l), Some(r)) => {
        if l < r {
          (l, r)
        } else {
          (r, l)
        }
      }
      c => panic!(
        "Expected bot {} to have full hands, but got: {:?}",
        full_hands_bot, c
      ),
    };
    bot.left_hand = None;
    bot.right_hand = None;
    let mut full_hands_bots = vec![];
    for (value, dest) in &[(lesser, bot.rule.low), (greater, bot.rule.high)] {
      match dest {
        Destination::Bot(idx) => {
          if self.bots[*idx as usize].take(*value) {
            full_hands_bots.push(*idx);
          }
        }
        Destination::Output(idx) => {
          self.outputs[*idx as usize].push(*value);
        }
      }
    }
    full_hands_bots
  }
}

fn simulate_bots(instructions: &str, want_little: u8, want_big: u8) -> (Option<u8>, State) {
  let (mut state, full_hands_bot) = State::new(instructions);
  let mut full_hands_bots = vec![full_hands_bot];
  let mut matched_bot: Option<u8> = None;
  while let Some(bot_idx) = full_hands_bots.pop() {
    let bot = &state.bots[bot_idx as usize];
    if (want_little, want_big) == sort_tuple((bot.left_hand.unwrap(), bot.right_hand.unwrap())) {
      assert!(matched_bot.is_none());
      matched_bot = Some(bot_idx);
    }
    full_hands_bots.append(&mut state.tick(bot_idx));
  }
  println!("{:#?}", state);
  (matched_bot, state)
}

fn sort_tuple(t: (u8, u8)) -> (u8, u8) {
  if t.0 < t.1 {
    t
  } else {
    (t.1, t.0)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(Some(2), simulate_bots(EXAMPLE, 2, 5).0);
  }

  #[test]
  fn my_input() {
    assert_eq!(Some(116), simulate_bots(MY_INPUT, 17, 61).0);
  }

  #[test]
  fn part_2_examples() {
    let (_, state) = simulate_bots(MY_INPUT, 17, 61);
    let result =
      state.outputs[0][0] as u64 * state.outputs[1][0] as u64 * state.outputs[2][0] as u64;
    assert_eq!(23903, result);
  }

  static EXAMPLE: &'static str = "\
    value 5 goes to bot 2\n\
    bot 2 gives low to bot 1 and high to bot 0\n\
    value 3 goes to bot 1\n\
    bot 1 gives low to output 1 and high to bot 0\n\
    bot 0 gives low to output 2 and high to output 0\n\
    value 2 goes to bot 2\n\
  ";
  static MY_INPUT: &'static str = include_str!("day_10_input.txt");
}
