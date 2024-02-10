use std::fs;
use twinwidth::algo::Greedy;
use twinwidth::graph::Graph;

//use twinwidth::{algo::Greedy, graph::Graph};
fn main() {
    // let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // d.push("pacechallenge/tiny-set");

    // println!("{}", d.display());

    // let files = fs::read_dir(d).unwrap();
    // for file in files {
    //     println!("Name: {}", file.unwrap().path().display())
    // }
    let file_graph = fs::read_to_string(
        "/Users/uwewarschun/Projects/twinwidth/pacechallenge/tiny-set/tiny001.gr",
    )
    .expect("File does not exist!");
    let graph = Graph::from_gr(&file_graph);

    let graphs = [graph];
    for _ in 0..100000 {
        for graph in &graphs {
            let mut greedy = Greedy::new_with_graph(graph.clone());

            let (_contraction_squence, _tw) = greedy.solve();

            //for contraction in contraction_squence {
            //println!("contraction: {:?}", contraction);
            //}
            if greedy.get_get_max_red_degree() == 1 {
                println!("{:?}", greedy.get_get_max_red_degree());
            }
            //println!("{:-<30}", "");
        }
    }
}
