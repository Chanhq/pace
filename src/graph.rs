use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    iter,
};

use crate::error::Error;

#[derive(Debug)]
pub struct Graph {
    number_of_nodes: usize,
    number_of_fixed_nodes: usize,
    number_of_free_nodes: usize,
    number_of_edges: usize,
    adjacency_list: Vec<HashSet<usize>>,
}
// GETTERS ---------------------------------------------------------------------------------
impl Graph {
    pub fn number_of_nodes(&self) -> usize {
        self.number_of_nodes
    }

    pub fn number_of_fixed_nodes(&self) -> usize {
        self.number_of_fixed_nodes
    }

    pub fn number_of_free_nodes(&self) -> usize {
        self.number_of_free_nodes
    }

    pub fn number_of_edges(&self) -> usize {
        self.number_of_edges
    }
}

// PUBLIC FUNCTIONS ---------------------------------------------------------------------------------
impl Graph {
    pub fn new(number_of_fixed_nodes: usize, number_of_free_nodes: usize) -> Graph {
        let number_of_nodes = number_of_fixed_nodes + number_of_free_nodes;
        Graph {
            number_of_nodes,
            number_of_fixed_nodes,
            number_of_free_nodes,
            number_of_edges: 0,
            adjacency_list: iter::repeat(HashSet::new()).take(number_of_nodes).collect(),
        }
    }

    pub fn add_edge(
        &mut self,
        fixed_node_index: usize,
        free_node_index: usize,
    ) -> Result<bool, Error> {
        self.check_fixed_node_index(fixed_node_index)?;
        self.check_free_node_index(free_node_index)?;

        let neighbors = self
            .adjacency_list
            .get_mut(fixed_node_index)
            .expect("fixed_node_index is valid, so it should be in bound");

        let inserted_successfully = neighbors.insert(free_node_index);

        if inserted_successfully {
            self.number_of_edges += 1;
        }

        Ok(inserted_successfully)
    }

    pub fn sort_fas(&self) -> Vec<usize> {
        let sorted_nodes = self.sort_nodes();
        let mut feedback_arc_set: Vec<usize> = Vec::new();
        let mut removed_nodes: HashSet<usize> = HashSet::new();

        for node_index in sorted_nodes {
            for neighbor_index in self
                .adjacency_list
                .get(node_index)
                .expect("Index must exist")
            {
                if removed_nodes.insert(*neighbor_index) {
                    feedback_arc_set.push(*neighbor_index);
                }
            }
        }

        feedback_arc_set
    }
}

// PRIVATE FUNCTIONS ---------------------------------------------------------------------------------
impl Graph {
    fn check_fixed_node_index(&self, index: usize) -> Result<(), Error> {
        if (0..self.number_of_fixed_nodes).contains(&index) {
            Ok(())
        } else {
            Err(Error::IndexError(format!(
                "Fixed index {index} is out of bounds"
            )))
        }
    }

    fn check_free_node_index(&self, index: usize) -> Result<(), Error> {
        if (self.number_of_fixed_nodes..self.number_of_nodes).contains(&index) {
            Ok(())
        } else {
            Err(Error::IndexError(format!(
                "Free index {index} is out of bounds"
            )))
        }
    }

    fn sort_nodes(&self) -> Vec<usize> {
        let mut in_degree: Vec<usize> = iter::repeat(0).take(self.number_of_nodes).collect();

        for neighbors in self.adjacency_list.iter() {
            for neighbor_index in neighbors {
                *in_degree.get_mut(*neighbor_index).expect(
                    "The generated vector must be large enough to fit all graph indices",
                ) += 1;
            }
        }

        let mut q: VecDeque<usize> = VecDeque::new();

        in_degree
            .iter()
            .enumerate()
            .filter(|(_, d)| **d == 0)
            .for_each(|(i, _)| q.push_front(i));

        let mut sorted_nodes: Vec<usize> = Vec::new();

        while !q.is_empty() {
            let next_index = q
                .pop_front()
                .expect("Element exists, because queue is not empty");
            sorted_nodes.push(next_index);

            for neighbor_index in self
                .adjacency_list
                .get(next_index)
                .expect("Index must exist")
            {
                let degree = in_degree
                    .get_mut(*neighbor_index)
                    .expect("The generated vector must be large enough to fit all graph indices");

                match *degree {
                    2.. => *degree -= 1,
                    1 => {
                        *degree -= 1;
                        q.push_back(*neighbor_index);
                    }
                    0 => panic!("The degree of a node would have been reduced "),
                }
            }
        }

        sorted_nodes
    }
}
