#![allow(dead_code)]

use colored::Colorize;
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Point(i64, i64);
impl Point {
  fn taxicab_distance(self, other: Self) -> u64 {
    ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u64
  }
}

#[derive(Clone, Copy)]
struct Maze {
  favorite_number: i64,
}
impl Maze {
  fn new(favorite_number: i64) -> Self {
    Self { favorite_number }
  }
  fn is_wall(&self, Point(x, y): Point) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + self.favorite_number).count_ones() % 2 == 1
  }

  fn min_path_between(&self, from: Point, to: Point) -> (BTreeSet<Point>, Option<u64>) {
    let mut heap = std::collections::BinaryHeap::new();
    let mut visited = std::collections::BTreeSet::new();
    heap.push(AStarPoint {
      steps_taken: 0,
      taxicab_distance: from.taxicab_distance(to),
      point: from,
    });
    while let Some(astar_point) = heap.pop() {
      if astar_point.point == to {
        return (visited, Some(astar_point.steps_taken));
      }
      for successor in self.successors(astar_point, to) {
        if visited.contains(&successor.point) {
          continue;
        }
        visited.insert(successor.point);
        heap.push(successor);
      }
    }
    return (visited, None);
  }

  fn count_locations_within_distance(&self, from: Point, within: u64) -> (BTreeSet<Point>, u64) {
    let mut count: u64 = 0;
    let mut vec = VecDeque::new();
    let mut visited = std::collections::BTreeSet::new();
    vec.push_back((0, from));
    visited.insert(from);
    while let Some((steps, point)) = vec.pop_front() {
      count += 1;
      for neighbor in self.neighbors(point) {
        if steps >= within {
          continue;
        }
        if visited.contains(&neighbor) {
          continue;
        }
        visited.insert(neighbor);
        vec.push_back((steps + 1, neighbor));
      }
    }
    (visited, count)
  }

  fn neighbors(&self, Point(x, y): Point) -> Vec<Point> {
    let mut result = Vec::with_capacity(4);
    if x != 0 {
      let point = Point(x - 1, y);
      if !self.is_wall(point) {
        result.push(point);
      }
    }
    if y != 0 {
      let point = Point(x, y - 1);
      if !self.is_wall(point) {
        result.push(point);
      }
    }
    let point = Point(x + 1, y);
    if !self.is_wall(point) {
      result.push(point);
    }
    let point = Point(x, y + 1);
    if !self.is_wall(point) {
      result.push(point);
    }
    result
  }

  fn successors(&self, current: AStarPoint, target: Point) -> Vec<AStarPoint> {
    let mut result = Vec::with_capacity(4);
    let Point(x, y) = current.point;
    let steps_taken = current.steps_taken + 1;
    if x != 0 {
      let point = Point(x - 1, y);
      if !self.is_wall(point) {
        result.push(AStarPoint {
          steps_taken,
          point,
          taxicab_distance: point.taxicab_distance(target),
        });
      }
    }
    if y != 0 {
      let point = Point(x, y - 1);
      if !self.is_wall(point) {
        result.push(AStarPoint {
          steps_taken,
          point,
          taxicab_distance: point.taxicab_distance(target),
        });
      }
    }
    let point = Point(x + 1, y);
    if !self.is_wall(point) {
      result.push(AStarPoint {
        steps_taken,
        point,
        taxicab_distance: point.taxicab_distance(target),
      });
    }
    let point = Point(x, y + 1);
    if !self.is_wall(point) {
      result.push(AStarPoint {
        steps_taken,
        point,
        taxicab_distance: point.taxicab_distance(target),
      });
    }
    result
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct AStarPoint {
  steps_taken: u64,
  taxicab_distance: u64,
  point: Point,
}
impl PartialOrd for AStarPoint {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for AStarPoint {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.steps_taken + self.taxicab_distance)
      .cmp(&(other.steps_taken + other.taxicab_distance))
      .reverse()
  }
}

struct VisualizedMaze {
  maze: Maze,
  visited: BTreeSet<Point>,
  height: i64,
  width: i64,
}
impl std::fmt::Display for VisualizedMaze {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        let point = Point(x, y);
        let wall = self.maze.is_wall(point);
        let visited = self.visited.contains(&point);
        if wall && visited {
          panic!("{:?} was both visited and a wall??");
        } else if wall {
          f.write_str("#")?;
        } else if visited {
          f.write_fmt(format_args!("{}", &"O".green()))?;
        } else {
          f.write_str(" ")?;
        }
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let maze = Maze::new(10);
    let (visited, dist) = maze.min_path_between(Point(1, 1), Point(7, 4));
    println!(
      "\n{}",
      VisualizedMaze {
        maze,
        visited,
        width: 10,
        height: 10
      }
    );
    assert_eq!(Some(11), dist);
  }

  #[test]
  fn my_input() {
    let maze = Maze::new(1358);
    assert_eq!(
      Some(96),
      maze.min_path_between(Point(1, 1), Point(31, 39)).1
    );
  }

  #[test]
  fn part_2_my_input() {
    let maze = Maze::new(1358);
    for i in 0..=50 {
      let (visited, result) = maze.count_locations_within_distance(Point(1, 1), i);
      println!(
        "\nFound {} distinct points within {}:\n{}",
        result,
        i,
        VisualizedMaze {
          maze,
          visited,
          width: 30,
          height: 30
        }
      );
    }
    let (_visited, result) = maze.count_locations_within_distance(Point(1, 1), 50);
    assert_eq!(141, result);
  }
}
