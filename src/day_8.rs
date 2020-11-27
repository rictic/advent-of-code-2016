#![allow(dead_code)]

use regex::Regex;
use std::fmt::Display;

struct LittleScreen {
  pixels: Vec<Vec<bool>>,
}
impl LittleScreen {
  fn example_sized() -> Self {
    Self {
      pixels: vec![vec![false; 7]; 3],
    }
  }

  fn full_sized() -> Self {
    Self {
      pixels: vec![vec![false; 50]; 6],
    }
  }

  fn take_commands(&mut self, commands: &str) {
    let commands = commands.lines().map(|command| Command::parse(command));
    for command in commands {
      self.take_command(command);
    }
  }

  fn take_command(&mut self, command: Command) {
    match command {
      Command::Rect { x, y } => {
        for y in 0..y {
          for x in 0..x {
            self.pixels[y as usize][x as usize] = true;
          }
        }
      }
      Command::RotateColumn { by, x } => {
        let len = self.pixels.len();
        let mut new_column = vec![false; len];
        for i in 0..len {
          let shifted_idx = (i + by) % len;
          new_column[shifted_idx] = self.pixels[i][x];
        }
        for (i, pixel) in new_column.into_iter().enumerate() {
          self.pixels[i][x] = pixel;
        }
      }
      Command::RotateRow { by, y } => {
        let len = self.pixels[0].len();
        let mut new_row = vec![false; len];
        for i in 0..len {
          let shifted_idx = (i + by) % len;
          new_row[shifted_idx] = self.pixels[y][i];
        }
        for (i, pixel) in new_row.into_iter().enumerate() {
          self.pixels[y][i] = pixel;
        }
      }
    }
  }

  fn pixels_lit(&self) -> usize {
    self
      .pixels
      .iter()
      .map(|row| row.iter().filter(|p| **p).count())
      .sum()
  }
}
impl Display for LittleScreen {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in self.pixels.iter() {
      for pixel_on in row.iter() {
        if *pixel_on {
          f.write_str("#")?;
        } else {
          f.write_str(".")?;
        }
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}

enum Command {
  Rect { x: usize, y: usize },
  RotateRow { y: usize, by: usize },
  RotateColumn { x: usize, by: usize },
}
impl Command {
  fn parse(str: &str) -> Self {
    lazy_static! {
      static ref RECT: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
      static ref ROW: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
      static ref COLUMN: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    }
    if let Some(captures) = RECT.captures(str) {
      return Command::Rect {
        x: captures[1].parse().unwrap(),
        y: captures[2].parse().unwrap(),
      };
    }
    if let Some(captures) = COLUMN.captures(str) {
      return Command::RotateColumn {
        x: captures[1].parse().unwrap(),
        by: captures[2].parse().unwrap(),
      };
    }
    if let Some(captures) = ROW.captures(str) {
      return Command::RotateRow {
        y: captures[1].parse().unwrap(),
        by: captures[2].parse().unwrap(),
      };
    }
    panic!("Could not parse command: {}", str);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let mut screen = LittleScreen::example_sized();
    assert_eq!(
      format!("{}", screen),
      ".......\n\
       .......\n\
       .......\n"
    );
    screen.take_command(Command::Rect { x: 3, y: 2 });
    assert_eq!(
      format!("{}", screen),
      "###....\n\
       ###....\n\
       .......\n"
    );
    screen.take_command(Command::RotateColumn { x: 1, by: 1 });
    assert_eq!(
      format!("{}", screen),
      "#.#....\n\
       ###....\n\
       .#.....\n"
    );
    screen.take_command(Command::RotateRow { y: 0, by: 4 });
    assert_eq!(
      format!("{}", screen),
      "....#.#\n\
       ###....\n\
       .#.....\n"
    );
    screen.take_command(Command::RotateColumn { x: 1, by: 1 });
    assert_eq!(
      format!("{}", screen),
      ".#..#.#\n\
       #.#....\n\
       .#.....\n"
    );
    assert_eq!(6, screen.pixels_lit());
  }

  #[test]
  fn example() {
    let mut screen = LittleScreen::example_sized();
    screen.take_commands(
      "rect 3x2\n\
       rotate column x=1 by 1\n\
       rotate row y=0 by 4\n\
       rotate column x=1 by 1",
    );
    assert_eq!(
      format!("{}", screen),
      ".#..#.#\n\
       #.#....\n\
       .#.....\n"
    );
    assert_eq!(6, screen.pixels_lit());
  }

  #[test]
  fn my_input() {
    let mut screen = LittleScreen::full_sized();
    screen.take_commands(MY_INPUT);
    println!("{}", screen);
    assert_eq!(
      format!("{}", screen),
      "####..##...##..###...##..###..#..#.#...#.##...##..\n\
       #....#..#.#..#.#..#.#..#.#..#.#..#.#...##..#.#..#.\n\
       ###..#..#.#..#.#..#.#....#..#.####..#.#.#..#.#..#.\n\
       #....#..#.####.###..#.##.###..#..#...#..####.#..#.\n\
       #....#..#.#..#.#.#..#..#.#....#..#...#..#..#.#..#.\n\
       ####..##..#..#.#..#..###.#....#..#...#..#..#..##..\n"
    );
    assert_eq!(128, screen.pixels_lit());
  }

  static MY_INPUT: &'static str = include_str!("day_8_input.txt");
}
