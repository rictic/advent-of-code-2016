#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Disc {
  size: u64,
  position: u64,
}
impl Disc {
  const fn new(size: u64, position: u64) -> Self {
    Self { size, position }
  }

  fn wait(&mut self, elapsed: u64) {
    self.position = self.position_in(elapsed);
  }

  fn position_in(&self, elapsed: u64) -> u64 {
    (self.position + elapsed) % self.size
  }
}

struct Sculpture {
  time: u64,
  discs: Vec<Disc>,
}
impl Sculpture {
  fn new(discs: Vec<Disc>) -> Self {
    Self { time: 0, discs }
  }

  fn wait(&mut self, elapsed: u64) {
    self.time += elapsed;
    for disc in self.discs.iter_mut() {
      disc.wait(elapsed);
    }
  }

  fn first_drop_time(&mut self) -> u64 {
    // Ok, this is hilariously unoptimized but it completes in <300ms?
    loop {
      if self.ball_can_drop() {
        return self.time;
      }
      self.wait(1);
    }
  }

  fn ball_can_drop(&self) -> bool {
    for (i, d) in self.discs.iter().enumerate() {
      let position = d.position_in(i as u64 + 1);
      if position != 0 {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE: [Disc; 2] = [Disc::new(5, 4), Disc::new(2, 1)];

  const MY_INPUT: [Disc; 6] = [
    Disc::new(17, 15),
    Disc::new(3, 2),
    Disc::new(19, 4),
    Disc::new(13, 2),
    Disc::new(7, 2),
    Disc::new(5, 0),
  ];

  #[test]
  fn examples() {
    let mut sculpture = Sculpture::new(EXAMPLE.iter().map(|c| *c).collect());
    assert_eq!(false, sculpture.ball_can_drop());
    sculpture.wait(5);
    assert_eq!(true, sculpture.ball_can_drop());

    let mut sculpture = Sculpture::new(EXAMPLE.iter().map(|c| *c).collect());
    assert_eq!(5, sculpture.first_drop_time());
  }

  #[test]
  fn my_input() {
    let mut sculpture = Sculpture::new(MY_INPUT.iter().map(|c| *c).collect());
    assert_eq!(400589, sculpture.first_drop_time());
  }

  #[test]
  fn part_2_example() {
    let mut discs: Vec<_> = MY_INPUT.iter().map(|c| *c).collect();
    discs.push(Disc::new(11, 0));
    let mut sculpture = Sculpture::new(discs);
    assert_eq!(3045959, sculpture.first_drop_time());
  }

  #[test]
  fn part_2_my_input() {}
}
