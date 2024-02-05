use std::{collections::HashSet, fmt::Debug, iter};

use super::Graph;

#[derive(Debug)]
pub struct PenaltyDigraph {
    number_of_nodes: usize,
    adjacency_list: Vec<HashSet<usize>>,
}

// CONSTRUCTORS
impl PenaltyDigraph {
    pub fn new(number_of_nodes: usize) -> PenaltyDigraph {
        let adjacency_list = iter::repeat(HashSet::new()).take(number_of_nodes).collect();

        PenaltyDigraph {
            number_of_nodes,
            adjacency_list,
        }
    }

    pub fn from_graph(graph: &Graph) -> PenaltyDigraph {
        let mut penalty_digraph = PenaltyDigraph::new(graph.number_of_free_nodes);

        for u in graph.number_of_fixed_nodes..graph.number_of_nodes {
            let mut neighbors_u: Vec<&usize> = graph
                .adjacency_list
                .get(u)
                .expect("Must exist")
                .iter()
                .collect();
            neighbors_u.sort();
            for v in u + 1..graph.number_of_nodes {
                let mut c_uv: isize = 0;
                let mut c_vu: isize = 0;
                let mut scan;
                let degree_v = graph.adjacency_list.get(v).expect("Must exist").len() as isize;

                let mut neighbors_v: Vec<&usize> = graph
                    .adjacency_list
                    .get(v)
                    .expect("Must exist")
                    .iter()
                    .collect();
                neighbors_v.sort();

                let mut adj_u_iter = neighbors_u.iter();
                let mut adj_u = adj_u_iter.next();

                let mut adj_v_iter = neighbors_v.iter();
                let mut adj_v = adj_v_iter.next();

                while adj_u.is_some() {
                    scan = 0;
                    while adj_v.is_some() && adj_v.unwrap() < adj_u.unwrap() {
                        adj_v = adj_v_iter.next();
                        scan += 1;
                    }
                    c_uv += scan;
                    c_vu = c_vu + degree_v - scan - 1;

                    if adj_u < adj_v {
                        c_vu += 1;
                    }
                    adj_u = adj_u_iter.next();
                }

                penalty_digraph.add_crossings(
                    u - graph.number_of_fixed_nodes,
                    v - graph.number_of_fixed_nodes,
                    c_uv,
                    c_vu,
                );
            }
        }

        penalty_digraph
    }
}

// PUBLIC METHODS
impl PenaltyDigraph {

    fn add_crossings(&mut self, u: usize, v: usize, c_uv: isize, c_vu: isize) {
        if c_vu < c_uv {
            self.add_edge(u, v);
        } else if c_uv < c_vu {
            self.add_edge(v, u);
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) -> bool {
        self.adjacency_list.get_mut(u).unwrap().insert(v)
    }

    fn edge_exists(&self, u: usize, v: usize) -> bool {
        self.adjacency_list.get(u).unwrap().contains(&v)
    }

    pub fn sort_fas(&self) -> Vec<usize> {
        let mut ordering: Vec<usize> = Vec::new();
        for u in 0..self.number_of_nodes {
            let mut val: isize = 0;
            let mut min: isize = 0;
            let mut loc: usize = u;

            for j in (0..loc).rev() {
                let v = ordering.get(j).expect("Index exists");
                if self.edge_exists(u, *v) {
                    val += 1;
                }
                if self.edge_exists(*v, u) {
                    val -= 1;
                }

                if val <= min {
                    min = val;
                    loc = j;
                }
            }
            ordering.insert(loc, u);
        }

        ordering
    }
}
