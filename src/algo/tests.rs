use crate::graph::Graph;

use super::Greedy;

#[test]
fn test_new_greedy_with_graph() {
    let graph = Graph::from_edges(vec![(1, 2), (2, 3)]);
    let greedy = Greedy::new_with_graph(graph);

    assert_eq!(greedy.contraction_squence, Vec::new());
    assert_eq!(greedy.graph, Graph::from_edges(vec![(1, 2), (2, 3)]));
}
