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

    _graph.add_node(1);

    assert_eq!(_graph.adj_set.len(), 1);
}

#[test]
fn test_add_two_nodes() {
    let mut _graph = Graph::new();

    _graph.add_node(1);
    _graph.add_node(2);

    assert_eq!(_graph.adj_set.len(), 2);
}

#[test]
fn test_add_edge() {
    let mut _graph = Graph::new();

    _graph.add_node(1);
    _graph.add_node(2);

    assert!(_graph.add_edge(1, 2));
    assert!(!_graph.add_edge(1, 2));

    if let Some(set) = _graph.adj_set.get(&1) {
        assert_eq!(set, &HashSet::from([2]));
    }

    if let Some(set) = _graph.adj_set.get(&2) {
        assert_eq!(set, &HashSet::from([1]));
    }
}

#[test]
#[should_panic]
fn test_add_edge_panic_first_node() {
    let mut graph: Graph = Graph::new();
    graph.add_node(2);
    graph.add_edge(1, 2);
}

#[test]
#[should_panic]
fn test_add_edge_panic_second_node() {
    let mut graph: Graph = Graph::new();
    graph.add_node(1);
    graph.add_edge(1, 2);
}

#[test]
fn test_get_neighbours() {
    let mut _graph = Graph::new();

    _graph.add_node(1);
    _graph.add_node(2);
    _graph.add_edge(1, 2);
    _graph.add_edge(1, 2);

    let neighbours_1 = _graph.get_neighbours(1);
    let neighbours_2 = _graph.get_neighbours(2);

    assert_eq!(neighbours_1, &HashSet::from([2]));
    assert_eq!(neighbours_2, &HashSet::from([1]));
}

#[test]
#[should_panic]
fn test_get_neighbours_panic() {
    let mut _graph = Graph::new();

    _graph.add_node(1);
    _graph.add_node(2);
    _graph.add_edge(1, 2);
    _graph.add_edge(1, 2);

    _graph.get_neighbours(3);
}
