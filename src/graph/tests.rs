use super::Graph;
use std::collections::HashSet;

#[test]
fn test_new_graph() {
    let _graph = Graph::new();

    assert_eq!(_graph.adj_set.len(), 0);
}

#[test]
fn test_add_one_node() {
    let mut _graph = Graph::new();

    _graph.add_node();

    assert_eq!(_graph.adj_set.len(), 1);
}

#[test]
fn test_add_two_nodes() {
    let mut _graph = Graph::new();

    _graph.add_node();
    _graph.add_node();

    assert_eq!(_graph.adj_set.len(), 2);
}

#[test]
fn test_add_edge() {
    let mut _graph = Graph::new();

    _graph.add_node();
    _graph.add_node();

    assert!(_graph.add_edge(0, 1));
    assert!(!_graph.add_edge(0, 1));

    assert_eq!(_graph.adj_set[0], HashSet::from([1]));
    assert_eq!(_graph.adj_set[1], HashSet::from([0]));
}
