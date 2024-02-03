use std::{fs, path, time::Instant};

use crate::{error::Error, graph::{self, Graph}, graph_builder::GraphBuilder};

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
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

    pub fn test(&self) -> Result<(), Error> {
        let graph = GraphBuilder::build_random_graph(3, 3, 9)?;
        let ordering = graph.sort_fas();

        let res = graph.compute_number_of_crossings_for_ordering(&ordering);

        println!("Crossings: {res:?}");

        Ok(())
    }
}

impl Application {
    fn run_test_on_graph_from_file(&self, filename: &str, should_print_ordering: bool, should_compute_number_of_crossings: bool) -> Result<(), Error> {
        println!("Loading Graph from file '{filename}'.");
        let begin = Instant::now();
        let graph = GraphBuilder::build_graph_from_file(filename)?;
        println!("Finished loading graph. ({} ms)", begin.elapsed().as_millis());
        println!("The graph has {} fixed nodes, {} free nodes and {} edges.", graph.number_of_fixed_nodes(), graph.number_of_free_nodes(), graph.number_of_edges());
        self.run_test_on_graph(&graph, should_print_ordering, should_compute_number_of_crossings)?;

        Ok(())
    }

    fn run_test_on_graph(&self, graph: &Graph, should_print_ordering: bool, should_compute_number_of_crossings: bool) -> Result<(), Error> {
        if should_compute_number_of_crossings {
            let begin = Instant::now();
            println!("Computing number of crossings for default ordering.");
            let number_of_crossings = graph.compute_number_of_crossings_with_default_ordering()?;
            println!("The graph has {} crossings. ({} ms)",number_of_crossings , begin.elapsed().as_millis());
        }

        let begin = Instant::now();
        println!("Computing ordering for free nodes.");
        let ordering = graph.sort_fas();
        println!("Finished computing ordering. ({} ms)", begin.elapsed().as_millis());

        if should_print_ordering {
            let ordering_with_actual_node_name: Vec<usize> = ordering.iter().map(|x| x + 1).collect();
            println!("Ordering: {:?}", ordering_with_actual_node_name);
        }

        if should_compute_number_of_crossings {
            let begin = Instant::now();
            println!("Computing number of crossings for computed ordering.");
            let number_of_crossings = graph.compute_number_of_crossings_for_ordering(&ordering)?;
            println!("The graph has {} crossings with the new ordering. ({} ms)",number_of_crossings , begin.elapsed().as_millis());
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
