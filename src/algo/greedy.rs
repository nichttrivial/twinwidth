use std::{cmp, collections::HashSet, fmt::Write};

use crate::algo::{get_all_combinations, Algo};
use crate::graph::Graph;

/// Holds a graph and its contraction squence.
/// In the beginning the contraction sequence is empty.
/// Each contraction on the graph will be stored in the contraction sequence in the occuring order.
/// The max red degree will be stored as well
pub struct Greedy {
    graph: Graph,
    global_red_edges: Graph,
    contraction_squence: Vec<(u32, u32)>,
    twin_width: usize,
}

impl Algo for Greedy {
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
    /// use twinwidth::algo::{Algo, greedy::Greedy};
    /// let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    /// let greedy = Greedy::new_with_graph(graph);
    /// ```
    fn new_with_graph(graph: Graph) -> Self {
        Greedy {
            graph,
            global_red_edges: Graph::new(),
            contraction_squence: Vec::new(),
            twin_width: 0,
        }
    }

    /// Gets the max red degree
    fn get_max_red_degree(&self) -> usize {
        self.twin_width
    }

    /// Performs the greedy algorithm
    ///
    /// # Returns
    /// * The contraction sequence after completly working through the given graph.
    ///
    /// # Example
    /// ```
    /// use twinwidth::graph::Graph;
    /// use twinwidth::algo::{Algo, greedy::Greedy};
    /// let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    /// let mut greedy = Greedy::new_with_graph(graph);
    /// let contraction_sequence = greedy.solve();
    /// ```
    fn solve(&mut self) -> (Vec<(u32, u32)>, usize) {
        while self.graph.get_all_nodes().len() > 1 {
            //TODO: Make this Option or smart in another way.
            let mut local_red_degree: usize = 100000;
            let mut contraction: (u32, u32) = (100000, 100000);
            let mut red_edges: Graph = Graph::new();

            let mut all_nodes = self.graph.get_all_nodes();
            //The use of Hashmap/Hashset implementation has no order, which indeed has effects on the result.
            all_nodes.sort();

            for (node_a, node_b) in get_all_combinations(all_nodes) {
                //prepare Graph for local red edges
                let mut local_red_edges = self.global_red_edges.clone();
                local_red_edges.add_node(node_a);
                local_red_edges.add_node(node_b);

                //get the neighbourhoods of the two edges and evalute the difference
                let neighbours_a = self.graph.get_neighbours(node_a);
                let neighbours_b = self.graph.get_neighbours(node_b);

                //TODO: PERFORMANCE! This is bad
                // -----------------------------------------------------------------------
                /*
                   This leads to many insert operations into the HashMap/HashSet Datastructure.
                   For sure it is bad idea to do it this way in a nCr2 for-loop embedded in a n while-loop
                   resulting in n * (n! / 2! * (n-2)!) iterations. 1000 Nodes => 1000 * 499.500 = 499.500.000
                   As one can see this may lead to billions of inserts really quick.

                   * Nevertheless, it would be interesting to see how far the data structures could be optimized. *

                   The diff will not change for most nodes over the n iterations. Therefore it is not neccesarry to calculate
                   all combinations every iteration and instead just update/recalculate the combinations wich might have changed.
                   GreedyV2 will go for this approach.
                */
                let diff: HashSet<_> = neighbours_a
                    .symmetric_difference(neighbours_b)
                    .filter(|item| item != &&node_a && item != &&node_b)
                    .collect();

                //The difference would create new red edges. Add them to the local red edges
                neighbours_a
                    .iter()
                    .filter(|x| diff.contains(x))
                    .for_each(|node| {
                        local_red_edges.add_node(*node);
                        local_red_edges.add_edge(node_a, *node);
                    });
                neighbours_b
                    .iter()
                    .filter(|x| diff.contains(x))
                    .for_each(|node| {
                        local_red_edges.add_node(*node);
                        local_red_edges.add_edge(node_b, *node);
                    });
                //-------------------------------------------------------------------------

                //Contract the nodes on the local red edges
                local_red_edges.contract_nodes(node_a, node_b);

                //Evalute the max degree of the local red edges and save preliminary result
                if local_red_degree > local_red_edges.get_max_degree() {
                    local_red_degree = local_red_edges.get_max_degree();
                    contraction = (node_a, node_b);
                    red_edges = local_red_edges;
                    if local_red_degree == 0 {
                        //We take the first best solution. And with 0 there cannot be some better
                        break;
                    }
                }
            }

            //Update Algo internals after each iteration
            self.global_red_edges = red_edges;
            self.twin_width = cmp::max(self.twin_width, local_red_degree);
            self.contraction_squence.push(contraction);
            self.graph.contract_nodes(contraction.0, contraction.1);
        }

        (self.contraction_squence.clone(), self.twin_width)
    }

    /// Constructs an string with resepect to the .tww format defined by the pace challenge
    /// See [Pace IO Definition](https://pacechallenge.org/2023/io/) for more information.
    ///
    /// # Returns
    /// A new string with respect to the tww format
    fn output_tww_str(&self) -> String {
        let mut tww = String::new();
        self.contraction_squence
            .iter()
            .for_each(|(node_a, node_b)| {
                writeln!(&mut tww, "{} {}", node_a, node_b).unwrap();
            });
        tww
    }
}

#[cfg(test)]
mod tests;
