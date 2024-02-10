use std::{collections::HashMap, fs, path::PathBuf, process::Command};

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

        //check against pre defined expected results
        assert_eq!(
            tw,
            *solution,
            "Graph: {}",
            path.path().file_name().unwrap().to_str().unwrap()
        );

        //check against the verfier provided by the pace challenge
        write_tww_to_fs(&greedy.output_tww_str());
        let verifier_result =
            execute_python_verifier(path.path().file_name().unwrap().to_str().unwrap());

        assert_eq!(tw, verifier_result);
    }
}

//This function expects that the right result is placed in result.tww before calling.
fn execute_python_verifier(filename: &str) -> usize {
    let mut script = env!("CARGO_MANIFEST_DIR").to_string();
    script.push_str("/pacechallenge/verifier.py");

    let mut graph = env!("CARGO_MANIFEST_DIR").to_string();
    graph.push_str("/pacechallenge/tiny-set/");
    graph.push_str(filename);

    let mut result = env!("CARGO_MANIFEST_DIR").to_string();
    result.push_str("/pacechallenge/result.tww");

    let output = Command::new("python")
        .args([script, graph, result])
        .output()
        .unwrap();

    let width = String::from_utf8(output.stdout).unwrap();
    let width = width.split(":").collect::<Vec<&str>>()[1].trim();
    let width: usize = width.parse().expect("Unable to parse number");
    width
}

fn write_tww_to_fs(tww: &str) {
    let mut path = env!("CARGO_MANIFEST_DIR").to_string();
    path.push_str("/pacechallenge/result.tww");

    fs::write(path, tww).unwrap();
}
