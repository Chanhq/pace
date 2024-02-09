use std::{collections::HashSet, fmt::Debug, iter};

use super::Graph;


/// Default representation of a penalty digraph for the OCM problem
/// 
/// This struct represents a directed graph that can be constructed by using a regular `Graph`.
/// When constructing this digraph from a regular `Graph`, the digraph can become cyclic.
/// 
/// In the conversion, each edge `a -> b` represents that the number of crossings caused by a node `a` and a node `b` are minimal, when node `a` is ordered before node `b`.
/// Solving the DFAS problem for this (maybe acyclic) digraph corresponds to solving the OCM problem on the regular `Graph`.
#[derive(Debug)]
pub struct PenaltyDigraph {
    number_of_nodes: usize,
    adjacency_list: Vec<HashSet<usize>>,
}

// CONSTRUCTORS
impl PenaltyDigraph {
    /// Constructs a new `PenaltyDigraph` without edges
    pub fn new(number_of_nodes: usize) -> PenaltyDigraph {
        let adjacency_list = iter::repeat(HashSet::new()).take(number_of_nodes).collect();

        PenaltyDigraph {
            number_of_nodes,
            adjacency_list,
        }
    }

    /// Constructs a new `PenaltyDigraph` by applying the algorithm described in [this paper](https://dl.acm.org/doi/abs/10.1145/945394.945396).
    pub fn from_graph(graph: &Graph) -> PenaltyDigraph {
        let mut penalty_digraph = PenaltyDigraph::new(graph.number_of_free_nodes);

        for u in graph.number_of_fixed_nodes..graph.number_of_nodes {
            for v in u + 1..graph.number_of_nodes {
                let mut c_uv: isize = 0;
                let mut c_vu: isize = 0;
                let mut scan;
                let degree_v = graph.adjacency_list.get(v).expect("Must exist").len() as isize;

                let mut adj_u_iter = graph.adjacency_list.get(u).unwrap().iter();
                let mut adj_u = adj_u_iter.next();

                let mut adj_v_iter = graph.adjacency_list.get(v).unwrap().iter();
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
    /// Adds an edge that depends on the crossing number between two nodes to the graph.
    /// 
    /// Always adds the edge in the direction with less crossings. 
    /// If the crossings in both orderings are equal, no edge is added.
    fn add_crossings(&mut self, u: usize, v: usize, c_uv: isize, c_vu: isize) {
        if c_vu < c_uv {
            self.add_edge(u, v);
        } else if c_uv < c_vu {
            self.add_edge(v, u);
        }
    }

    // Adds an edge between two nodes
    fn add_edge(&mut self, u: usize, v: usize) -> bool {
        self.adjacency_list.get_mut(u).unwrap().insert(v)
    }

    /// Checks, if an edge between two nodes exists
    fn edge_exists(&self, u: usize, v: usize) -> bool {
        self.adjacency_list.get(u).unwrap().contains(&v)
    }

    /// Computes an ordering that would solve the DFAS problem
    /// 
    /// This algorithm is described in [this paper](https://arxiv.org/pdf/2208.09234.pdf)
    pub fn sort_fas(&self) -> Vec<usize> {
        let mut feedback_arc_set: Vec<usize> = Vec::new();
        for u in 0..self.number_of_nodes {
            let mut val: isize = 0;
            let mut min: isize = 0;
            let mut loc: usize = u;

            for j in (0..loc).rev() {
                let v = feedback_arc_set.get(j).expect("Index exists");
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
            feedback_arc_set.insert(loc, u);
        }

        feedback_arc_set
    }
}
