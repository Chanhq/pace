use std::{
    cmp::Ordering,
    fs::{self, File},
    io::{self, Write},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    graph::{
        self,
        penalty_digraph::{self, PenaltyDigraph},
        Graph,
    },
    graph_builder::GraphBuilder,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkStats {
    pub number_of_fixed_nodes: usize,
    pub number_of_free_nodes: usize,
    pub number_of_edges: usize,
    pub generation_duration: u128,
    pub ordering_duration: u128,
}

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
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

        let mut number_of_graphs_with_crossings: usize = 0;
        let mut crossing_counts: Vec<usize> = Vec::new();

        for _ in 0..number_of_test_cases {
            let graph =
                GraphBuilder::build_graph_with_fixed_nodes_and_no_crossings(number_of_fixed_nodes);
            let ordering = graph.sort_fas();
            let number_of_crossings = graph.compute_number_of_crossings_for_ordering(&ordering)?;
            if number_of_crossings != 0 {
                number_of_graphs_with_crossings += 1;
                crossing_counts.push(number_of_crossings);
            }
        }

        println!(
            "{} of 5000 graphs had some crossings after solving.",
            number_of_graphs_with_crossings
        );
        println!("Crossings found: {:?}", crossing_counts);

        Ok(())
    }

    pub fn run_benchmark_for_20_million_edges(&self) -> Result<Vec<BenchmarkStats>, Error> {
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        println!("----- Starting benchmark for 20 million edges and different number of nodes -------------------------------------------------");

        benchmark_stats.push(self.run_test_on_randomly_generated_graph(5_000, 5_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(10_000, 10_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(15_000, 15_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(20_000, 20_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(30_000, 30_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(40_000, 40_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(50_000, 50_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(60_000, 60_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(70_000, 70_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(80_000, 80_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(90_000, 90_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(100_000, 100_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(150_000, 150_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(200_000, 200_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(250_000, 250_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(300_000, 300_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(400_000, 400_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(500_000, 500_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(750_000, 750_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_250_000, 1_250_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_500_000, 1_500_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(2_000_000, 2_000_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(2_500_000, 2_500_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(3_000_000, 3_000_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(3_500_000, 3_500_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(4_000_000, 4_000_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(5_000_000, 5_000_000, 20_000_000)?);

        let mut file = File::create("benchmark_20_million_edges.json")?;
        let json = serde_json::to_string_pretty(&benchmark_stats).expect("Converted to json.");
        let _ = file.write_all(json.as_bytes());

        Ok(benchmark_stats)
    }

    pub fn run_benchmark_for_1_million_fixed_and_free_nodes(
        &self,
    ) -> Result<Vec<BenchmarkStats>, Error> {
        let mut benchmark_stats: Vec<BenchmarkStats> = Vec::new();

        println!("----- Starting benchmark for 1 million fixed nodes, 1 millon free nodes and different number of edges -------------------------------------------------");

        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 100_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 500_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 1_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 2_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 3_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 4_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 5_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 10_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 20_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 30_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 40_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 50_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 60_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 70_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 80_000_000)?);
        benchmark_stats
            .push(self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 90_000_000)?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(
            1_000_000,
            1_000_000,
            100_000_000,
        )?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(
            1_000_000,
            1_000_000,
            150_000_000,
        )?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(
            1_000_000,
            1_000_000,
            200_000_000,
        )?);
        benchmark_stats.push(self.run_test_on_randomly_generated_graph(
            1_000_000,
            1_000_000,
            250_000_000,
        )?);

        let mut file = File::create("benchmark_1_million_fixed_and_free_nodes.json")?;
        let json = serde_json::to_string_pretty(&benchmark_stats).expect("Converted to json.");
        let _ = file.write_all(json.as_bytes());

        Ok(benchmark_stats)
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
    ) -> Result<(), Error> {
        print!("Loading graph from file '{}'...", filename);
        io::stdout().flush()?;
        let begin = Instant::now();
        let graph = GraphBuilder::build_graph_from_file(filename)?;
        println!(" done! ({} ms)", begin.elapsed().as_millis());
        println!(
            "The graph has {} fixed nodes, {} free nodes and {} edges.",
            graph.number_of_fixed_nodes(),
            graph.number_of_free_nodes(),
            graph.number_of_edges()
        );
        self.run_test_on_graph(
            &graph,
            should_print_ordering,
            should_compute_number_of_crossings,
        )?;

        Ok(())
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
        let generation_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", generation_elapsed);

        let begin = Instant::now();
        print!("Computing ordering...");
        io::stdout().flush()?;
        graph.sort_fas();
        let ordering_elapsed = begin.elapsed().as_millis();
        println!(" done! ({} ms)", generation_elapsed);
        println!("");

        Ok(BenchmarkStats {
            number_of_fixed_nodes,
            number_of_free_nodes,
            number_of_edges,
            generation_duration: generation_elapsed,
            ordering_duration: ordering_elapsed,
        })
    }

    fn run_test_on_graph(
        &self,
        graph: &Graph,
        should_print_ordering: bool,
        should_compute_number_of_crossings: bool,
    ) -> Result<(), Error> {
        let begin = Instant::now();
        print!("Generating cross table...");
        io::stdout().flush()?;
        let penalty_digraph = PenaltyDigraph::from_graph(&graph);
        println!(" done! ({} ms)", begin.elapsed().as_millis());

        // println!("{:#?}", penalty_digraph);

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
            .iter()
            .map(|e| e + graph.number_of_fixed_nodes())
            .collect();
        println!(" done! ({} ms)", begin.elapsed().as_millis());

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

        Ok(())
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
