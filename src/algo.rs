//! This module contains algorithms to solve the twinwidth problem

use crate::graph::Graph;

/// Holds a graph and its contraction squence.
/// In the beginning the contraction sequence is empty.
/// Each contraction on the graph will be stored in the contraction sequence in the occuring order.
pub struct Greedy {
    graph: Graph,
    contraction_squence: Vec<(u32, u32)>,
}

impl Greedy {
    /// Creates a new `Greedy` instance
    ///
    /// # Parameters
    /// * graph: The graph on wich the greedy algorithm should be performed.
    /// 
    /// # Returns
    /// * New Greedy instance with a graph and empty contraction sequence
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// use twinwidth::algo::Greedy;
    /// let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    /// let greedy = Greedy::new_with_graph(graph);
    /// ```
    pub fn new_with_graph(graph: Graph) -> Self {
        Greedy {
            graph,
            contraction_squence: Vec::new(),
        }
    }

    /// Performs the greedy algorithm
    ///
    /// # Returns
    /// * The contraction sequence after completly working through the given graph.
    ///
    /// # Example
    /// ```
    /// use twinwidth::graph::Graph;
    /// use twinwidth::algo::Greedy;
    /// let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    /// let mut greedy = Greedy::new_with_graph(graph);
    /// let contraction_sequence = greedy.solve();
    /// ```
    pub fn solve(&mut self) -> Vec<(u32, u32)> {
        self.contraction_squence.clone()
    }
}

#[cfg(test)]
mod tests;
