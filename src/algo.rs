//! This module contains algorithms to solve the twinwidth problem

use crate::graph::Graph;

/// Holds a graph and its contraction squence.
/// In the beginning the contraction sequence is empty.
/// Each contraction on the graph will be stored in the contraction sequence in the occuring order.
/// The max red degree will be stored as well
pub struct Greedy {
    graph: Graph,
    contraction_squence: Vec<(u32, u32)>,
    max_red_degree: usize,
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
            max_red_degree: 0,
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
        //TODO: Make this Option or find another smart way for initialisation issues
        let mut min_diff = 10000;
        let mut contraction = (10000,10000);

        let nodes = self.graph.get_all_nodes();

        //TODO: 1|2 is the same as 2|1, so for->for is dumb... 
        for node_a in &nodes {
            for node_b in &nodes {
                if node_a == node_b {
                    continue;
                }

                let neighbours_a = self.graph.get_neighbours(*node_a);
                let neighbours_b = self.graph.get_neighbours(*node_b);

                let diff = neighbours_a
                    .symmetric_difference(neighbours_b)
                    .filter(|item| item != &node_a && item != &node_b)
                    .count();
                
                //TODO: Take already exisiting red edges into account
                if diff < min_diff {
                    min_diff = diff;
                    contraction = (*node_a, *node_b);
                }
            }
        }

        self.graph.contract_nodes(contraction.0, contraction.1);
        self.contraction_squence.push(contraction);
        if self.max_red_degree < min_diff {
            self.max_red_degree = min_diff;
        }

        //TODO: abandon recursion, Maybe ? Profile differences ?
        if nodes.len() > 2 {
            self.solve();
        }

        self.contraction_squence.clone()
    }
}

#[cfg(test)]
mod tests;
