# PACE2024
Repository for the [PACE challenge 2024](https://pacechallenge.org/2024/).

### Approach
Reduction to DFAS (Directed Feedback Arc Set) Problem.

General idea: load directed graph with adjacency list, reduce the OCR problem to DFAS and solve the DFAS problem with one of the existing algorithms.
The reduction to a cyclic graph can be found in [this paper](https://dl.acm.org/doi/abs/10.1145/945394.945396), while the algorithm to compute the directed feedback arc set is described in [this paper](https://arxiv.org/pdf/2208.09234.pdf).

### How to use
Run the `cargo run --release` command in order to run all generated test cases (This might take a couple hours tho, because the number of test instances are rather large)
Run the `cargo doc --open` command to generate visualized documentation and open it in your standard-browser.
