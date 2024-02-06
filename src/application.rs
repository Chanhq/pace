use std::{
    fs::{self, File},
    io::{self, Write},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    graph::{penalty_digraph::PenaltyDigraph, Graph},
    graph_builder::GraphBuilder,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkStats {
    pub number_of_fixed_nodes: usize,
    pub number_of_free_nodes: usize,
    pub number_of_edges: usize,
    pub loading_elapsed: u128,
    pub reduction_elapsed: u128,
    pub ordering_elapsed: u128,
}

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
    }

    pub fn run_tests_with_same_edges(&self) -> Result<Vec<BenchmarkStats>, Error> {
        let number_of_edges = 100_000;
        let node_step_size = 500;
        let max_number_of_nodes = 30_000;

        println!("----- Run tests on graphs with {number_of_edges} edges --------------------------------------------------");
        let mut file = File::create("benchmark_results/benchmark_with_const_edges.json")?;
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        file.write(b"[\n")?;
        for number_of_nodes in (node_step_size..= max_number_of_nodes).step_by(node_step_size) {
            let benchmark = self.run_test_on_randomly_generated_graph(number_of_nodes, number_of_nodes, number_of_edges)?;
            let benchmark_json = serde_json::to_string_pretty(&benchmark).expect("Converted to json.");
            file.write(benchmark_json.as_bytes())?;
            file.write(b",\n")?;
            benchmark_stats.push(benchmark);
        }
        file.write(b"]")?;

        Ok(benchmark_stats)
    }

    pub fn run_tests_with_same_edges_and_free_nodes(&self) -> Result<Vec<BenchmarkStats>, Error> {
        let number_of_free_nodes = 5_000;
        let number_of_edges = 50_000;
        let fixed_node_step_size = 5_000;
        let max_number_of_fixed_nodes = 500_000;

        println!("----- Run tests on graphs with {number_of_free_nodes} free nodes and {number_of_edges} edges --------------------------------------------------");
        let mut file = File::create("benchmark_results/benchmark_with_const_free_nodes_and_edges.json")?;
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        file.write(b"[\n")?;
        for number_of_fixed_nodes in (fixed_node_step_size..= max_number_of_fixed_nodes).step_by(fixed_node_step_size) {
            let benchmark = self.run_test_on_randomly_generated_graph(number_of_fixed_nodes, number_of_free_nodes, number_of_edges)?;
            let benchmark_json = serde_json::to_string_pretty(&benchmark).expect("Converted to json.");
            file.write(benchmark_json.as_bytes())?;
            file.write(b",\n")?;
            benchmark_stats.push(benchmark);
        }
        file.write(b"]")?;

        Ok(benchmark_stats)
    }

    pub fn run_tests_with_same_nodes(&self) -> Result<Vec<BenchmarkStats>, Error> {
        let number_of_nodes = 5_000;
        let edge_step_size = 10_000;
        let max_number_of_edges = 2_000_000;

        println!("----- Run tests on graphs with {number_of_nodes} fixed and free nodes --------------------------------------------------");
        let mut file = File::create("benchmark_results/benchmark_with_const_nodes.json")?;
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        file.write(b"[\n")?;
        for number_of_edges in (edge_step_size..= max_number_of_edges).step_by(edge_step_size) {
            let benchmark = self.run_test_on_randomly_generated_graph(number_of_nodes, number_of_nodes, number_of_edges)?;
            let benchmark_json = serde_json::to_string_pretty(&benchmark).expect("Converted to json.");
            file.write(benchmark_json.as_bytes())?;
            file.write(b",\n")?;
            benchmark_stats.push(benchmark);
        }
        file.write(b"]")?;

        Ok(benchmark_stats)
    }

    pub fn run_small_tests(&self) -> Result<Vec<BenchmarkStats>, Error> {
        println!("----- Run some random tests --------------------------------------------------");
        let mut file = File::create("benchmark_results/benchmark_random_tests.json")?;
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        benchmark_stats.push(self.run_test_on_randomly_generated_graph(50, 50, 200)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(100, 100, 2000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(200, 200, 4000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(500, 500, 10000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(1000, 1000, 50000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(2000, 2000, 100000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(5000, 5000, 250000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(10000, 10000, 1000000)?);

        let json = serde_json::to_string_pretty(&benchmark_stats).expect("Converted to json.");
        let _ = file.write_all(json.as_bytes());

        Ok(benchmark_stats)
    }

    pub fn run_on_specific_case(
        &self,
        number_of_fixed_nodes: usize,
        number_of_test_cases: usize,
    ) -> Result<(), Error> {
        println!("----- Running on test graph that should have no crossings when ordered correctly -------------------------------------------------");
        println!(
            "Testing {} graphs with {} fixed nodes and {} free nodes.",
            number_of_test_cases, number_of_fixed_nodes, number_of_fixed_nodes
        );

        let mut crossing_counts: Vec<usize> = Vec::new();

        for _ in 0..number_of_test_cases {
            let graph =
                GraphBuilder::build_graph_with_fixed_nodes_and_no_crossings(number_of_fixed_nodes);
            let penalty_digraph = PenaltyDigraph::from_graph(&graph);
            let ordering = penalty_digraph
                .sort_fas()
                .iter()
                .map(|e| e + graph.number_of_fixed_nodes())
                .collect();
            let number_of_crossings = graph.compute_number_of_crossings_for_ordering(&ordering)?;
            if number_of_crossings != 0 {
                crossing_counts.push(number_of_crossings);
            }
        }

        println!(
            "{} of {} graphs had some crossings after solving.",
            crossing_counts.len(),
            number_of_test_cases,
        );
        println!("Crossings found: {:?}", crossing_counts);

        Ok(())
    }

    pub fn run_tests_on_tiny_test_set(&self) -> Result<(), Error> {
        let paths = fs::read_dir("ressources/tiny_test_set")?;

        let mut filenames: Vec<String> = paths
            .flatten()
            .map(|entry| entry.path().to_str().unwrap().to_owned())
            .collect();
        filenames.sort();

        for filename in filenames {
            self.run_test_on_graph_from_file(&filename, true, true)?;
        }

        Ok(())
    }
}

impl Application {
    fn run_test_on_graph_from_file(
        &self,
        filename: &str,
        should_print_ordering: bool,
        should_compute_number_of_crossings: bool,
    ) -> Result<BenchmarkStats, Error> {
        print!("Loading graph from file '{}'...", filename);
        io::stdout().flush()?;
        let begin = Instant::now();
        let graph = GraphBuilder::build_graph_from_file(filename)?;
        let loading_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", loading_elapsed);
        println!(
            "The graph has {} fixed nodes, {} free nodes and {} edges.",
            graph.number_of_fixed_nodes(),
            graph.number_of_free_nodes(),
            graph.number_of_edges()
        );

        self.run_test_on_graph(
            &graph,
            loading_elapsed,
            should_print_ordering,
            should_compute_number_of_crossings,
        )
    }

    fn run_test_on_randomly_generated_graph(
        &self,
        number_of_fixed_nodes: usize,
        number_of_free_nodes: usize,
        number_of_edges: usize,
    ) -> Result<BenchmarkStats, Error> {
        let begin = Instant::now();
        print!(
            "Generating graph with {} fixed nodes, {} free nodes and {} edges...",
            number_of_fixed_nodes, number_of_free_nodes, number_of_edges
        );
        io::stdout().flush()?;
        let graph = GraphBuilder::build_random_graph(
            number_of_fixed_nodes,
            number_of_free_nodes,
            number_of_edges,
        )?;
        let loading_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", loading_elapsed);

        self.run_test_on_graph(&graph, loading_elapsed, false, false)
    }

    fn run_test_on_graph(
        &self,
        graph: &Graph,
        loading_elapsed: u128,
        should_print_ordering: bool,
        should_compute_number_of_crossings: bool,
    ) -> Result<BenchmarkStats, Error> {
        let begin = Instant::now();
        print!("Computing penalty digraph...");
        io::stdout().flush()?;
        let penalty_digraph = PenaltyDigraph::from_graph(&graph);
        let reduction_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", reduction_elapsed);

        if should_compute_number_of_crossings {
            let begin = Instant::now();
            print!("Computing number of crossings for default ordering...");
            io::stdout().flush()?;
            let number_of_crossings = graph.compute_number_of_crossings_with_default_ordering()?;
            println!(" done! ({} ms)", begin.elapsed().as_millis());
            println!("The graph has {} crossings.", number_of_crossings);
        }

        let begin = Instant::now();
        print!("Computing ordering for free nodes...");
        io::stdout().flush()?;
        let ordering: Vec<usize> = penalty_digraph
            .sort_fas()
            .into_iter()
            .map(|e| e + graph.number_of_fixed_nodes())
            .collect();
        let ordering_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", ordering_elapsed);

        if should_print_ordering {
            let ordering_with_actual_node_name: Vec<usize> =
                ordering.iter().map(|x| x + 1).collect();
            println!("Ordering: {:?}", ordering_with_actual_node_name);
        }

        if should_compute_number_of_crossings {
            let begin = Instant::now();
            print!("Computing number of crossings for computed ordering...");
            io::stdout().flush()?;
            let number_of_crossings = graph.compute_number_of_crossings_for_ordering(&ordering)?;
            println!(" done! ({} ms)", begin.elapsed().as_millis());
            println!(
                "The graph has {} crossings with the new ordering.",
                number_of_crossings
            );
        }

        println!("");

        Ok(BenchmarkStats {
            number_of_fixed_nodes: graph.number_of_fixed_nodes(),
            number_of_free_nodes: graph.number_of_free_nodes(),
            number_of_edges: graph.number_of_edges(),
            loading_elapsed,
            reduction_elapsed,
            ordering_elapsed,
        })
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
