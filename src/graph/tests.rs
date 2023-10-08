use super::Graph;
use std::collections::HashSet;

#[test]
fn test_newgraph() {
    let graph = Graph::new();

    assert_eq!(graph.adj_set.len(), 0);
}

#[test]
fn test_add_one_node() {
    let mut graph = Graph::new();

    graph.add_node(1);

    assert_eq!(graph.adj_set.len(), 1);
}

#[test]
fn test_add_two_nodes() {
    let mut graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);

    assert_eq!(graph.adj_set.len(), 2);
}

#[test]
fn test_add_edge() {
    let mut graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);

    assert!(graph.add_edge(1, 2));
    assert!(!graph.add_edge(1, 2));

    if let Some(set) = graph.adj_set.get(&1) {
        assert_eq!(set, &HashSet::from([2]));
    }

    if let Some(set) = graph.adj_set.get(&2) {
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
    let mut graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);
    graph.add_edge(1, 2);
    graph.add_edge(1, 2);

    let neighbours_1 = graph.get_neighbours(1);
    let neighbours_2 = graph.get_neighbours(2);

    assert_eq!(neighbours_1, &HashSet::from([2]));
    assert_eq!(neighbours_2, &HashSet::from([1]));
}

#[test]
#[should_panic]
fn test_get_neighbours_panic() {
    let mut graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);
    graph.add_edge(1, 2);
    graph.add_edge(1, 2);

    graph.get_neighbours(3);
}

#[test]
fn test_contract_nodes() {
    let mut graph: Graph = Graph::new();

    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);

    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    assert_eq!(graph.adj_set.len(), 4);

    let neighbours = graph.get_neighbours(2);
    assert_eq!(neighbours, &HashSet::from([1, 3]));

    graph.contract_nodes(2, 3);

    assert_eq!(graph.adj_set.len(), 3);

    let neighbours = graph.get_neighbours(2);
    assert_eq!(neighbours, &HashSet::from([1, 4]));
}

#[test]
#[should_panic]
fn test_contract_nodes_panic_first_node() {
    let mut graph: Graph = Graph::new();

    graph.add_node(2);

    graph.contract_nodes(1, 2);
}

#[test]
#[should_panic]
fn test_contract_nodes_panic_second_node() {
    let mut graph: Graph = Graph::new();

    graph.add_node(1);

    graph.contract_nodes(1, 2);
}
