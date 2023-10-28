use crate::graph::Graph;

use super::Greedy;

#[test]
fn test_new_greedy_with_graph() {
    let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    let greedy = Greedy::new_with_graph(graph);

    assert_eq!(greedy.contraction_squence, Vec::new());
    assert_eq!(greedy.graph, Graph::from_edges(vec![(1, 2), (2, 3)]));
}

#[test]
fn test_solve_two_nodes() {
    let graph = Graph::from_edges(vec![(1, 2)]);
    let mut greedy = Greedy::new_with_graph(graph);

    greedy.solve();

    assert!(greedy.contraction_squence.iter().all(|item| vec![(1, 2),(2, 1)].contains(item)));
    assert_eq!(greedy.max_red_degree, 0);
}

#[test]
fn test_solve_x_nodes_cograph() {
    let mut graph = Graph::from_edges(vec![(1, 2), (2, 3), (3, 4), (4, 1)]);
    graph.add_node(5);
    let mut greedy = Greedy::new_with_graph(graph);

    greedy.solve();

    assert_eq!(greedy.contraction_squence.len(), 4);
    assert_eq!(greedy.graph.get_all_nodes().len(), 1);
    assert_eq!(greedy.graph.get_neighbours(greedy.graph.get_all_nodes()[0]).len(), 0);
    assert_eq!(greedy.max_red_degree, 0);
}

#[test]
fn test_solve_x_nodes_no_cograph() {
    let graph = Graph::from_edges(vec![(1, 2), (2, 3), (3, 4), (4, 1), (5, 2)]);
    let mut greedy = Greedy::new_with_graph(graph);

    greedy.solve();

    assert_eq!(greedy.contraction_squence.len(), 4);
    assert_eq!(greedy.graph.get_all_nodes().len(), 1);
    assert_eq!(greedy.graph.get_neighbours(greedy.graph.get_all_nodes()[0]).len(), 0);
    assert_eq!(greedy.max_red_degree, 1);
}