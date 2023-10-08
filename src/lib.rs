pub mod graph {
    //! This modules contains stuff for basic graph tasks
    use std::collections::{HashSet, HashMap};

    ///A Graph implementation using adjacency HashSet
    ///
    /// The idea behind using a HashSet is to make the comparison of neighbourhoods easy
    /// with set operations.
    pub struct Graph {
        adj_set: HashMap<u32, HashSet<u32>>,
    }

    impl Graph {
        /// Creates an empty `Graph`.
        ///
        /// # Returns
        /// * New Graph with empty HashMap
        /// 
        /// # Examples
        /// ```
        /// use twinwidth::graph::Graph;
        /// let graph: Graph = Graph::new();
        /// ```
        pub fn new() -> Graph {
            Graph {
                adj_set: HashMap::new(),
            }
        }

        /// Adds a new node without any edges to the graph
        /// 
        /// # Examples
        /// ```
        /// use twinwidth::graph::Graph;
        /// let mut graph: Graph = Graph::new();
        /// graph.add_node(1);
        /// ```
        pub fn add_node(&mut self, node: u32) {
            self.adj_set.insert(node, HashSet::new());
        }

        /// Adds a new edge between two nodes
        /// 
        /// # Panics
        /// If one egde does not exist
        /// 
        /// # Returns
        /// * returns true if the edge was newly created
        /// * returns false if the edge already existed
        ///
        /// # Examples
        /// ```
        /// use twinwidth::graph::Graph;
        /// let mut graph: Graph = Graph::new();
        /// graph.add_node(1);
        /// graph.add_node(2);
        /// graph.add_edge(1, 2);
        /// ```
        pub fn add_edge(&mut self, node_a: u32, node_b: u32) -> bool {
            let success_a =  match self.adj_set.get_mut(&node_a) {
                Some(set) => set.insert(node_b),
                None => panic!("Node does not exist"),
            };

            let success_b =  match self.adj_set.get_mut(&node_b) {
                Some(set) => set.insert(node_a),
                None => panic!("Node does not exist"),
            };

            success_a | success_b
        }
    }

    #[cfg(test)]
    mod tests;
}
