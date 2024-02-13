use std::path::PathBuf;
use twinwidth::algo::greedy::Greedy;
use twinwidth::analyzer::Analyzer;

//use twinwidth::{algo::Greedy, graph::Graph};
fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("pacechallenge/tiny-set");

    let mut analyzer: Analyzer<Greedy> = Analyzer::new();
    analyzer.run(path);

    analyzer.show_result();
}
