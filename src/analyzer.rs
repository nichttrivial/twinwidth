//! This module contains utility to analyze a given algorithm
//! Vision: * compare mulitple implementaitons or configurations from various alogrithm
//!         * provide some kind of probe which can collect metrics from inside of the algorithm
//!         * provide some export to excel/python/whatever to furhter analyze the results individaully
//!         * calculate multiple graphs parralel und give some progress information
use crate::{algo::Algo, graph::Graph};
use std::{fs, marker::PhantomData, path::PathBuf, time::Instant};

struct Metrics(f32);

#[derive(Default)]
pub struct Analyzer<T: Algo> {
    runs: Vec<(String, usize, Metrics)>,
    phantom: PhantomData<T>,
}

impl<T: Algo> Analyzer<T> {
    /// New analyzer instance
    pub fn new() -> Self {
        Analyzer {
            runs: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Runs the Analyzer with the given configuration (at the moment the path with .gr files)
    pub fn run(&mut self, path: PathBuf) {
        let mut files: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
        files.sort_by_key(|file| file.path());

        for file in files {
            let content = fs::read_to_string(file.path()).unwrap();

            let graph = Graph::from_gr(&content);
            let mut algo = T::new_with_graph(graph);

            let now = Instant::now();
            algo.solve();
            let elapsed = now.elapsed().as_secs_f32();

            let filename = file.file_name().to_str().unwrap().to_string();

            self.runs
                .push((filename, algo.get_max_red_degree(), Metrics(elapsed)));
        }
    }

    pub fn show_result(&self) {
        println!("{:-<30}", "");
        for run in &self.runs {
            println!("{}, {}, {}", run.0, run.1, run.2 .0);
        }
        println!("{:-<30}", "");
    }
}
