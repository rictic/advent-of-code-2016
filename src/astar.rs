#![allow(dead_code)]

use std::collections::BinaryHeap;

pub trait AStarSearcher {
  type Node: Sized + Ord + Copy;

  fn search(&mut self, initial: Self::Node) -> Option<(u64, Self::Node)> {
    let mut heap: BinaryHeap<SearchNode<Self::Node>> = BinaryHeap::new();
    // let mut seen = BTreeSet::new();
    heap.push(SearchNode {
      steps_so_far: 0,
      heuristic: self.optimistic_distance(&initial),
      node: initial,
    });
    while let Some(state) = heap.pop() {
      if state.heuristic == 0 {
        return Some((state.steps_so_far, state.node));
      }
      for successor in self.successors(&state.node) {
        heap.push(SearchNode {
          steps_so_far: state.steps_so_far + 1,
          node: successor,
          heuristic: self.optimistic_distance(&successor),
        });
      }
    }
    return None;
  }

  fn optimistic_distance(&self, node: &Self::Node) -> u64;
  fn successors(&mut self, node: &Self::Node) -> Vec<Self::Node>;
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct SearchNode<T>
where
  T: Sized + Eq,
{
  steps_so_far: u64,
  heuristic: u64,
  node: T,
}
impl<T> PartialOrd for SearchNode<T>
where
  T: Sized + Eq,
{
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl<T> Ord for SearchNode<T>
where
  T: Sized + Eq,
{
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.heuristic + self.steps_so_far)
      .cmp(&(other.heuristic + other.steps_so_far))
      .reverse()
  }
}
