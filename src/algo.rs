//! This module contains algorithms to solve the twinwidth problem

use crate::graph::Graph;
use itertools::{Itertools, TupleCombinations};

pub trait Algo {
    fn new_with_graph(graph: Graph) -> Self;

    fn get_max_red_degree(&self) -> usize;

    fn solve(&mut self) -> (Vec<(u32, u32)>, usize);

    fn output_tww_str(&self) -> String;
}

/// Holds a graph and its contraction squence.
/// In the beginning the contraction sequence is empty.
/// Each contraction on the graph will be stored in the contraction sequence in the occuring order.
/// The max red degree will be stored as well

fn get_all_combinations(nodes: Vec<u32>) -> TupleCombinations<std::vec::IntoIter<u32>, (u32, u32)> {
    let result: TupleCombinations<std::vec::IntoIter<u32>, (u32, u32)> =
        nodes.into_iter().tuple_combinations::<(u32, u32)>();
    result
}

pub mod greedy;
