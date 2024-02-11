use std::fs;
use std::path::PathBuf;
use twinwidth::algo::{Algo, Greedy};
use twinwidth::graph::Graph;

//use twinwidth::{algo::Greedy, graph::Graph};
fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("pacechallenge/tiny-set/tiny005.gr");
    let file_graph = fs::read_to_string(path).expect("File does not exist!");
    let graph = Graph::from_gr(&file_graph);

    for it in 0..1 {
        let mut greedy = Greedy::new_with_graph(graph.clone());

        let (_contraction_squence, tw) = greedy.solve();

        //for contraction in contraction_squence {
        //println!("contraction: {:?}", contraction);
        //}
        println!("{}, TW: {:?}", it, tw);
        if greedy.get_max_red_degree() == 3 {
            break;
        }
        //println!("{:-<30}", "");
    }
}
