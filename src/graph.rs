pub mod penalty_digraph;

use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
};


use crate::error::Error;

use self::penalty_digraph::PenaltyDigraph;

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
        node_index1: usize,
        node_index2: usize,
    ) -> Result<bool, Error> {
        if node_index1 >= self.number_of_nodes || node_index2 >= self.number_of_nodes {
            return Err(Error::IndexError("Index out of bounds".to_string()));
        }

        let neighbors1 = self
            .adjacency_list
            .get_mut(node_index1)
            .expect("fixed_node_index is valid, so it should be in bound");

        let inserted_successfully1 = neighbors1.insert(node_index2);

        let neighbors2 = self
            .adjacency_list
            .get_mut(node_index2)
            .expect("free_node_index is valid, so it should be in bound");

        let inserted_successfully2 = neighbors2.insert(node_index1);

        if inserted_successfully1 && inserted_successfully2 {
            self.number_of_edges += 1;
        }

        Ok(inserted_successfully1)
    }

    pub fn does_edge_exist(&self, index1: usize, index2: usize) -> Result<bool, Error> {
        if index1 >= self.number_of_nodes || index2 >= self.number_of_nodes {
            return Err(Error::IndexError("Index is out of bounds".to_string()));
        }

        Ok(self
            .adjacency_list
            .get(index1)
            .expect("Index exists")
            .contains(&index2))
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

    pub fn compute_number_of_crossings_with_default_ordering(&self) -> Result<usize, Error> {
        let mut number_of_crossings = 0;

        for fixed_node_index1 in 0..self.number_of_fixed_nodes {
            for fixed_node_index2 in (fixed_node_index1 + 1)..self.number_of_fixed_nodes {
                for neighbor_index1 in self
                    .adjacency_list
                    .get(fixed_node_index1)
                    .expect("Index must exist")
                {
                    for neighbor_index2 in self
                        .adjacency_list
                        .get(fixed_node_index2)
                        .expect("Index must exist")
                    {
                        if neighbor_index2 < neighbor_index1 {
                            number_of_crossings += 1;
                        }
                    }
                }
            }
        }

        Ok(number_of_crossings)
    }

    pub fn compute_number_of_crossings_for_ordering(
        &self,
        ordering: &Vec<usize>,
    ) -> Result<usize, Error> {
        if ordering.len() != self.number_of_free_nodes {
            return Err(Error::ValueError(
                "The ordering does not contain all free nodes".to_string(),
            ));
        }
        let included_indices: HashSet<usize> = ordering.iter().cloned().collect();
        if included_indices != (self.number_of_fixed_nodes..self.number_of_nodes).collect() {
            return Err(Error::ValueError(
                "The ordering does not contain all free nodes".to_string(),
            ));
        }

        let mut positions = HashMap::new();
        for (position, free_node_index) in ordering.iter().enumerate() {
            positions.insert(*free_node_index, position);
        }

        let mut number_of_crossings = 0;
        for fixed_node_index1 in 0..self.number_of_fixed_nodes {
            for fixed_node_index2 in (fixed_node_index1 + 1)..self.number_of_fixed_nodes {
                for neighbor_index1 in self
                    .adjacency_list
                    .get(fixed_node_index1)
                    .expect("Index must exist")
                {
                    for neighbor_index2 in self
                        .adjacency_list
                        .get(fixed_node_index2)
                        .expect("Index must exist")
                    {
                        let position1 = positions
                            .get(neighbor_index1)
                            .expect("A position must have been found");
                        let position2 = positions
                            .get(neighbor_index2)
                            .expect("A position must have been found");

                        if position2 < position1 {
                            number_of_crossings += 1;
                        }
                    }
                }
            }
        }

        Ok(number_of_crossings)
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
            .for_each(|(i, _)| q.push_back(i));

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
