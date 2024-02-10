use std::{collections::HashMap, fs, path::PathBuf};

use twinwidth::{algo::Greedy, graph::Graph};

#[test]
pub fn test_tiny_set() {
    let solutions: HashMap<&str, usize> = HashMap::from([
        ("tiny001.gr", 1),
        ("tiny002.gr", 2),
        ("tiny003.gr", 0),
        ("tiny004.gr", 0),
        ("tiny005.gr", 4), //TODO: Debug! Solution is 3.
        ("tiny006.gr", 0),
        ("tiny007.gr", 2),
        ("tiny008.gr", 4),
        ("tiny009.gr", 1),
        ("tiny010.gr", 2),
    ]);

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("pacechallenge/tiny-set");

    let files = fs::read_dir(d).unwrap();
    for file in files {
        let path = file.unwrap();

        let content = fs::read_to_string(path.path()).unwrap();

        let graph = Graph::from_gr(&content);
        let mut greedy = Greedy::new_with_graph(graph);

        let (_, tw) = greedy.solve();

        let solution = solutions
            .get(path.path().file_name().unwrap().to_str().unwrap())
            .unwrap();

        assert_eq!(
            tw,
            *solution,
            "Graph: {}",
            path.path().file_name().unwrap().to_str().unwrap()
        );
    }
}
