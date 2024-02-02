use std::time::Instant;

use crate::{error::Error, graph_builder::GraphBuilder};

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
}
