//! This module contains utility to analyze a given algorithm
//! Vision: * compare mulitple implementaitons or configurations from various alogrithm
//!         * provide some kind of probe which can collect metrics from inside of the algorithm
//!         * provide some export to excel/python/whatever to furhter analyze the results individaully
use crate::{algo::Algo, graph::Graph};
use std::{fs, path::PathBuf, time::Instant};

struct Metrics(f32);

#[derive(Default)]
pub struct Analyzer<T: Algo> {
    runs: Vec<(String, T, Metrics)>,
}

impl<T: Algo> Analyzer<T> {
    /// New analyzer instance
    pub fn new() -> Self {
        Analyzer { runs: Vec::new() }
    }

    /// Runs the Analyzer with the given configuration (at the moment the path with .gr files)
    pub fn run(&mut self, path: PathBuf) {
        let files = fs::read_dir(path).unwrap();
        for file in files {
            let path = file.unwrap();

            let content = fs::read_to_string(path.path()).unwrap();

            let graph = Graph::from_gr(&content);
            let mut greedy = T::new_with_graph(graph);

            let now = Instant::now();
            greedy.solve();
            let elapsed = now.elapsed().as_secs_f32();

            let filename = path.file_name().to_str().unwrap().to_string();
            self.runs.push((filename, greedy, Metrics(elapsed)));
        }
    }

    pub fn show_result(&self) {
        println!("{:-<30}", "");
        for run in &self.runs {
            println!("{}, {}, {}", run.0, run.1.get_max_red_degree(), run.2 .0);
        }
        println!("{:-<30}", "");
    }
}
