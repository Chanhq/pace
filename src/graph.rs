pub mod penalty_digraph;

use std::{collections::{BTreeSet, HashMap, HashSet}, iter};

use crate::error::Error;

#[derive(Debug)]
pub struct Graph {
    number_of_nodes: usize,
    number_of_fixed_nodes: usize,
    number_of_free_nodes: usize,
    number_of_edges: usize,
    adjacency_list: Vec<BTreeSet<usize>>,
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
            adjacency_list: iter::repeat(BTreeSet::new()).take(number_of_nodes).collect(),
        }
    }

    pub fn add_edge(&mut self, node_index1: usize, node_index2: usize) -> Result<bool, Error> {
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
