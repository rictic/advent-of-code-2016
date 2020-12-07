#![allow(dead_code)]

use std::collections::{BTreeSet, BinaryHeap};

pub trait AStarSearcher: Sized {
  type Node: Sized + Ord;
  type Successors: IntoIterator<Item=Self::Node> + std::iter::FromIterator<Self::Node>;

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
      for successor in self.successors(&state.node).into_iter() {
        let heuristic = self.optimistic_distance(&successor);
        heap.push(SearchNode {
          steps_so_far: state.steps_so_far + 1,
          node: successor,
          heuristic,
        });
      }
    }
    return None;
  }

  fn caching(self) -> CachingSearcher<Self> {
    CachingSearcher {
      searcher: self,
      seen: Default::default(),
    }
  }

  fn optimistic_distance(&self, node: &Self::Node) -> u64;
  fn successors(&mut self, node: &Self::Node) -> Self::Successors;
}

pub struct CachingSearcher<Searcher>
where
  Searcher: AStarSearcher,
{
  searcher: Searcher,
  pub seen: BTreeSet<Searcher::Node>,
}
impl<Searcher> AStarSearcher for CachingSearcher<Searcher>
where
  Searcher: AStarSearcher,
  Searcher::Node: Copy,
{
  type Node = Searcher::Node;
  type Successors = Searcher::Successors;

  fn optimistic_distance(&self, node: &Self::Node) -> u64 {
    self.searcher.optimistic_distance(node)
  }

  fn successors(&mut self, node: &Self::Node) -> Self::Successors {
    self
      .searcher
      .successors(node)
      .into_iter()
      .filter(|n| {
        let has_seen = self.seen.contains(n);
        if !has_seen {
          self.seen.insert(*n);
        }
        !has_seen
      })
      .collect()
  }
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
