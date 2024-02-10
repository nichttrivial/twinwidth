//! This modules contains stuff for basic graph tasks
use std::collections::{HashMap, HashSet};

/// A Graph implementation using adjacency HashSet
///
/// The idea behind using a HashSet is to make the comparison of neighbourhoods easy
/// with set operations.
#[derive(PartialEq, Debug, Clone, Default)]
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
    pub fn new() -> Self {
        Graph {
            adj_set: HashMap::new(),
        }
    }

    /// Creates an `Graph` from a Vector of edges.
    ///
    /// # Parameters
    /// * edges: A vector of edges
    ///
    /// # Returns
    /// * New Graph with edges
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// let edges = vec![(1,2),(2,3),(3,4)];
    /// let graph: Graph = Graph::from_edges(edges);
    /// ```
    pub fn from_edges(edges: Vec<(u32, u32)>) -> Self {
        let mut g = Self::new();
        for (node_a, node_b) in edges {
            if !g.adj_set.contains_key(&node_a) {
                g.add_node(node_a);
            }

            if !g.adj_set.contains_key(&node_b) {
                g.add_node(node_b);
            }

            g.add_edge(node_a, node_b);
        }
        g
    }

    /// Reads graph from a string following the .gr format.
    /// See [Pace IO Definition](https://pacechallenge.org/2023/io/) for more information.
    ///
    /// **The function assumes that your provided string is valid!**
    ///
    /// # Parameter
    /// * gr: string slice containing the
    ///
    /// # Returns
    /// * A new graph instance with respect to the gr-string.
    ///
    /// # Example
    /// ```
    /// use twinwidth::graph::Graph;
    ///let gr = "1 2\n\
    ///          2 3";
    ///let graph = Graph::from_gr(gr);
    /// ```
    pub fn from_gr(gr: &str) -> Self {
        //TODO: REFACTOR!
        let mut edges: Vec<(u32, u32)> = Vec::new();

        for line in gr.lines() {
            let mut parts = line.split_whitespace().map(|s| s.parse::<u32>());
            let edge: (u32, u32) = match (parts.next(), parts.next()) {
                (Some(Ok(a)), Some(Ok(b))) => (a, b),
                _ => continue,
            };

            edges.push(edge);
        }
        Self::from_edges(edges)
    }

    /// Adds a new node without any edges to the graph
    ///
    /// # Parameters
    /// * node: The node to add to the graph
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// let mut graph: Graph = Graph::new();
    /// graph.add_node(1);
    /// ```
    pub fn add_node(&mut self, node: u32) {
        if self.adj_set.get(&node).is_some() {
            return;
        }
        self.adj_set.insert(node, HashSet::new());
    }

    /// Adds a new edge between two nodes
    ///
    /// # Parameters
    /// * node_a: the first node of the edge
    /// * node_b: the second node of the edge
    ///
    /// # Returns
    /// * returns true if the edge was newly created
    /// * returns false if the edge already existed
    ///
    /// # Panics
    /// If one node does not exist
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
    /// # Parameters
    /// * node: The node wich neighbours should be returned
    ///
    /// # Returns
    /// * Reference to the HashSet of adjacent nodes
    ///
    /// # Panics
    /// * If the node does not exist
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
    /// # Parameters
    /// * node_a: The frist node of the contraction
    /// * node_b: The second node of the contraction
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
        //TODO: This implementation seems not very idomatic. Refactor!
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

        for adj in self.adj_set.values_mut() {
            //TODO: Do not remove, change to node a
            if adj.contains(&node_b) {
                adj.remove(&node_b);
                adj.insert(node_a);
            }
        }
    }

    /// Gets all existing nodes from the graph
    ///
    /// # Return
    /// * Returns a vector containig all nodes
    ///
    /// # Examples
    /// ```
    /// use twinwidth::graph::Graph;
    /// let mut graph: Graph = Graph::new();
    ///
    /// graph.add_node(1);
    /// graph.add_node(2);
    ///
    /// let nodes = graph.get_all_nodes();
    /// ```
    pub fn get_all_nodes(&self) -> Vec<u32> {
        self.adj_set.keys().cloned().collect()
    }

    /// Gets the max degree of the graph
    /// TODO: Implement tests and complete documentation
    pub fn get_max_degree(&self) -> usize {
        if let Some(max_degree) = self.adj_set.values().map(|value| value.len()).max() {
            return max_degree;
        }

        0
    }
}

#[cfg(test)]
mod tests;
