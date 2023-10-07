pub mod graph {
    use std::collections::HashSet;

    ///A Graph implementation using adjacency HashSet
    ///
    /// The idea behind using a HashSet is to make the comparison of neighbourhoods easy
    /// with set operations.
    pub struct Graph {
        adj_set: Vec<HashSet<usize>>,
    }

    impl Graph {
        /// Creates an empty `Graph`.
        ///
        /// # Examples
        /// ```
        /// use twinwidth::graph::Graph;
        /// let graph: Graph = Graph::new();
        /// ```
        pub fn new() -> Graph {
            Graph {
                adj_set: Vec::new(),
            }
        }

        pub fn add_node(&mut self) {
            self.adj_set.push(HashSet::new());
        }

        pub fn add_edge(&mut self, vert_a: usize, vert_b: usize) -> bool {
            self.adj_set[vert_a].insert(vert_b) | self.adj_set[vert_b].insert(vert_a)
        }
    }

    #[cfg(test)]
    mod tests;
}
