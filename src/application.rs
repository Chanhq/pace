use std::{
    fs::{self, File},
    io::{LineWriter, Write},
    path,
    time::Instant,
};

use crate::{
    error::Error,
    graph::{self, Graph},
    graph_builder::GraphBuilder,
};

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
    }

    pub fn run_benchmark_for_20_million_edges(&self) -> Result<(), Error> {
        let file = File::create("benchmark_20_million_edges.txt")?;
        let mut file = LineWriter::new(file);

        println!("----- Starting benchmark for 20 million edges and different number of nodes -------------------------------------------------");

        self.run_test_on_randomly_generated_graph(5_000, 5_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(10_000, 10_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(15_000, 15_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(20_000, 20_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(30_000, 30_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(40_000, 40_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(50_000, 50_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(60_000, 60_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(70_000, 70_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(80_000, 80_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(90_000, 90_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(100_000, 100_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(150_000, 150_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(200_000, 200_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(250_000, 250_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(300_000, 300_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(400_000, 400_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(500_000, 500_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(750_000, 750_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_250_000, 1_250_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_500_000, 1_500_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(2_000_000, 2_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(2_500_000, 2_500_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(3_000_000, 3_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(3_500_000, 3_500_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(4_000_000, 4_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(5_000_000, 5_000_000, 20_000_000, &mut file)?;

        Ok(())
    }

    pub fn run_benchmark_for_1_million_fixed_and_free_nodes(&self) -> Result<(), Error> {
        let file = File::create("benchmark_1_million_fixed_and_free_nodes.txt")?;
        let mut file = LineWriter::new(file);

        println!("----- Starting benchmark for 1 million fixed nodes, 1 millon free nodes and different number of edges -------------------------------------------------");

        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 100_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 500_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 1_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 2_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 3_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 4_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 5_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 30_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 40_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 60_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 70_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 80_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 90_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 100_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 150_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 200_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 250_000_000, &mut file)?;

        Ok(())
    }

    pub fn run_test_on_randomly_generated_graphs(&self) -> Result<(), Error> {
        let file = File::create("benchmark_results.txt")?;
        let mut file = LineWriter::new(file);

        self.run_test_on_randomly_generated_graph(5_000, 5_000, 2_500_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(10_000, 10_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(50_000, 50_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(100_000, 10_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(500_000, 500_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 10_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 20_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 30_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 50_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 100_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 250_000_000, &mut file)?;
        self.run_test_on_randomly_generated_graph(1_000_000, 1_000_000, 500_000_000, &mut file)?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), Error> {
        let begin = Instant::now();
        println!("Start generating graph.");
        let graph = GraphBuilder::build_random_graph(1_000_000, 1_000_000, 10_000_000)?;
        let elapsed = begin.elapsed().as_millis();
        println!("Finished generating graph with {} fixed nodes, {} free nodes and {} edges. ({elapsed} ms)", graph.number_of_fixed_nodes(), graph.number_of_free_nodes(), graph.number_of_edges());

        let begin = Instant::now();
        println!("Start computing ordering.");
        let _ordering = graph.sort_fas();
        let elapsed = begin.elapsed().as_millis();
        println!("Finished computing ordering. ({elapsed} ms)");

        Ok(())
    }

    pub fn run_tests_on_tiny_test_set(&self) -> Result<(), Error> {
        let paths = fs::read_dir("ressources/tiny_test_set")?;

        for filename in paths.flatten().map(|entry| entry.path()) {
            self.run_test_on_graph_from_file(filename.to_str().unwrap(), true, true)?;
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
        println!("Loading Graph from file '{filename}'.");
        let begin = Instant::now();
        let graph = GraphBuilder::build_graph_from_file(filename)?;
        println!(
            "Finished loading graph. ({} ms)",
            begin.elapsed().as_millis()
        );
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
        file: &mut LineWriter<File>,
    ) -> Result<(), Error> {
        let begin = Instant::now();
        println!(
            "Generating graph with {} fixed nodes, {} free nodes and {} edges.",
            number_of_fixed_nodes, number_of_free_nodes, number_of_edges
        );
        let graph = GraphBuilder::build_random_graph(
            number_of_fixed_nodes,
            number_of_free_nodes,
            number_of_edges,
        )?;
        let generation_elapsed = begin.elapsed().as_millis();
        println!("Finished generating graph. ({} ms)", generation_elapsed);

        let begin = Instant::now();
        println!("Computing ordering.");
        graph.sort_fas();
        let ordering_elapsed = begin.elapsed().as_millis();
        println!("Finished computing ordering. ({} ms)", ordering_elapsed);
        println!("");

        write!(
            file,
            "#fixed: {}\n#free: {}\n#edges: {}\ngeneration: {} ms\nordering: {} ms\n\n",
            graph.number_of_fixed_nodes(),
            graph.number_of_free_nodes(),
            graph.number_of_edges(),
            generation_elapsed,
            ordering_elapsed
        )?;

        Ok(())
    }

    fn run_test_on_graph(
        &self,
        graph: &Graph,
        should_print_ordering: bool,
        should_compute_number_of_crossings: bool,
    ) -> Result<(), Error> {
        if should_compute_number_of_crossings {
            let begin = Instant::now();
            println!("Computing number of crossings for default ordering.");
            let number_of_crossings = graph.compute_number_of_crossings_with_default_ordering()?;
            println!(
                "The graph has {} crossings. ({} ms)",
                number_of_crossings,
                begin.elapsed().as_millis()
            );
        }

        let begin = Instant::now();
        println!("Computing ordering for free nodes.");
        let ordering = graph.sort_fas();
        println!(
            "Finished computing ordering. ({} ms)",
            begin.elapsed().as_millis()
        );

        if should_print_ordering {
            let ordering_with_actual_node_name: Vec<usize> =
                ordering.iter().map(|x| x + 1).collect();
            println!("Ordering: {:?}", ordering_with_actual_node_name);
        }

        if should_compute_number_of_crossings {
            let begin = Instant::now();
            println!("Computing number of crossings for computed ordering.");
            let number_of_crossings = graph.compute_number_of_crossings_for_ordering(&ordering)?;
            println!(
                "The graph has {} crossings with the new ordering. ({} ms)",
                number_of_crossings,
                begin.elapsed().as_millis()
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
