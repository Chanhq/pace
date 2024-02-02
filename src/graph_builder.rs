use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rand::Rng;

use crate::{error::Error, graph::Graph};

pub struct GraphBuilder {}

// PUBLIC METHODS ------------------------------------------------------------------
impl GraphBuilder {
    pub fn build_graph_from_file(filename: &str) -> Result<Graph, Error> {
        let file = File::open(filename)?;

        let mut lines = BufReader::new(file).lines().flatten();
        let p_line = lines
            .find(|line| line.starts_with('p'))
            .ok_or(Error::ParseError(
                "Could not find a valid p line in the file".to_string(),
            ))?;

        let p_line_info = PLineInfo::build(&p_line).ok_or(Error::ParseError(
            "The found p-line was invalid".to_string(),
        ))?;

        let mut graph = Graph::new(
            p_line_info.number_of_fixed_nodes,
            p_line_info.number_of_free_nodes,
        );

        for line in lines {
            if line.starts_with('c') || line.is_empty() {
                continue;
            }
            let edge = GraphBuilder::parse_edge_line(&line)
                .ok_or(Error::ParseError("Unexpected line found!".to_string()))?;

            graph.add_edge(edge.0 - 1, edge.1 - 1)?;
        }

        if p_line_info.number_of_edges != graph.number_of_edges() {
            return Err(Error::ParseError(format!("The number of edges in the file was invalid. {} were expected, but {} were actually found.", p_line_info.number_of_edges, graph.number_of_edges())));
        }

        Ok(graph)
    }

    pub fn build_random_graph(
        number_of_fixed_nodes: usize,
        number_of_free_nodes: usize,
        number_of_edges: usize,
    ) -> Result<Graph, Error> {
        if let Some(maximum_number_of_edges) =
            number_of_fixed_nodes.checked_mul(number_of_free_nodes)
        {
            if number_of_edges > maximum_number_of_edges {
                return Err(Error::ValueError(
                    "It is not possible to construct the graph with that many edges".to_string(),
                ));
            }
        }

        let mut graph = Graph::new(number_of_fixed_nodes, number_of_free_nodes);
        let mut rng = rand::thread_rng();

        for _ in 0..number_of_edges {
            loop {
                let fixed_node_index = rng.gen_range(0..graph.number_of_fixed_nodes());
                let free_node_index =
                    rng.gen_range(graph.number_of_fixed_nodes()..graph.number_of_nodes());

                if graph.add_edge(fixed_node_index, free_node_index)? {
                    break;
                }
            }
        }

        Ok(graph)
    }
}

impl GraphBuilder {
    fn parse_edge_line(line: &str) -> Option<(usize, usize)> {
        let words: Vec<&str> = line.split(' ').collect();

        if words.len() != 2 {
            return None;
        }

        let fixed_option = words.get(0).unwrap().parse::<usize>().ok();
        let free_option = words.get(1).unwrap().parse::<usize>().ok();

        if fixed_option.is_none() || free_option.is_none() {
            return None;
        }

        Some((fixed_option.unwrap(), free_option.unwrap()))
    }
}

struct PLineInfo {
    descriptor: String,
    number_of_fixed_nodes: usize,
    number_of_free_nodes: usize,
    number_of_edges: usize,
}

impl PLineInfo {
    fn build(p_line: &str) -> Option<PLineInfo> {
        if !p_line.starts_with("p") {
            return None;
        }

        let words: Vec<&str> = p_line.split(' ').collect();

        if words.len() != 5 {
            return None;
        }

        let descriptor = words.get(1).unwrap().to_string();
        let number_of_fixed_nodes = words.get(2).unwrap().parse::<usize>().ok();
        let number_of_free_nodes = words.get(3).unwrap().parse::<usize>().ok();
        let number_of_edges = words.get(4).unwrap().parse::<usize>().ok();

        if number_of_fixed_nodes.is_none()
            || number_of_free_nodes.is_none()
            || number_of_edges.is_none()
        {
            return None;
        }

        Some(PLineInfo {
            descriptor,
            number_of_fixed_nodes: number_of_fixed_nodes.unwrap(),
            number_of_free_nodes: number_of_free_nodes.unwrap(),
            number_of_edges: number_of_edges.unwrap(),
        })
    }
}
