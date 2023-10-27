//! This modules contains stuff for basic graph tasks
use std::collections::{HashMap, HashSet};

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
    /// If one node does not exist
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
        let success_a = match self.adj_set.get_mut(&node_a) {
            Some(set) => set.insert(node_b),
            None => panic!("Node does not exist"),
        };

        let success_b = match self.adj_set.get_mut(&node_b) {
            Some(set) => set.insert(node_a),
            None => panic!("Node does not exist"),
        };

        success_a | success_b
    }

    /// Gets a reference to the HashSet of adjacent nodes
    ///
    /// # Panics
    /// * If the node does not exist
    ///
    /// # Returns
    /// * Reference to the HashSet of adjacent nodes
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// let mut graph: Graph = Graph::new();
    /// graph.add_node(1);
    /// graph.add_node(2);
    /// graph.add_edge(1, 2);
    /// graph.get_neighbours(1);
    /// ```
    pub fn get_neighbours(&self, node: u32) -> &HashSet<u32> {
        match self.adj_set.get(&node) {
            Some(set) => set,
            None => panic!("Node does not exist"),
        }
    }

    /// Contracts two nodes
    /// The emerging node will be saved under node_a.
    /// node_b will be deleted.
    ///
    /// # Panics
    /// If one node does not exist
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// let mut graph: Graph = Graph::new();
    ///
    /// graph.add_node(1);
    /// graph.add_node(2);
    /// graph.add_node(3);
    /// graph.add_node(4);
    ///
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    /// graph.add_edge(3, 4);
    ///
    /// graph.contract_nodes(2, 3);
    /// ```
    pub fn contract_nodes(&mut self, node_a: u32, node_b: u32) {
        let mut set_a = match self.adj_set.get(&node_a).cloned() {
            Some(set) => set,
            None => panic!("Node does not exist"),
        };
        set_a.remove(&node_b);

        let mut set_b = match self.adj_set.get(&node_b).cloned() {
            Some(set) => set,
            None => panic!("Node does not exist"),
        };
        set_b.remove(&node_a);

        let union: HashSet<u32> = set_a.union(&set_b).copied().collect();
        self.adj_set.insert(node_a, union);

        self.adj_set.remove(&node_b);
    }
}

#[cfg(test)]
mod tests;