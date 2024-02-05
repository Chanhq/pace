use pace::{application::Application, graph::{self, penalty_digraph::PenaltyDigraph, Graph}, graph_builder::GraphBuilder};


fn main(){
    // let mut graph = Graph::new(3, 4);
    // graph.add_edge(0, 3).unwrap();
    // graph.add_edge(0, 4).unwrap();
    // graph.add_edge(1, 3).unwrap();
    // graph.add_edge(1, 4).unwrap();
    // graph.add_edge(1, 5).unwrap();
    // graph.add_edge(2, 4).unwrap();
    // graph.add_edge(2, 5).unwrap();
    // graph.add_edge(2, 6).unwrap();

    // let mut graph = GraphBuilder::build_graph_from_file("ressources/tiny_test_set/cycle_8_sorted.gr").unwrap();

    // println!("Graph: {:#?}", graph);
    // let mut penalty = PenaltyDigraph::from_graph(&graph);
    // println!("CT: {:#?}", penalty);

    // let ordering = penalty.sort_fas().iter().map(|x| x + graph.number_of_fixed_nodes()).collect();

    // println!("ordering: {:?}", ordering);

    // let crossings_before = graph.compute_number_of_crossings_with_default_ordering().unwrap();
    // let crossings_after = graph.compute_number_of_crossings_for_ordering(&ordering).unwrap();

    // println!("{} -> {}", crossings_before, crossings_after);

    let application = Application::new();
    // application.run_on_specific_case().unwrap();

    application.run_tests_on_tiny_test_set().unwrap();
    
    // let graph = GraphBuilder::build_graph_from_file("ressources/tiny_test_set/cycle_8_shuffled.gr").unwrap();
    // let ordering = graph.new_sort_fas();
    // println!("{:?}", ordering);


    // application.run_benchmark_for_1_million_fixed_and_free_nodes().unwrap();
    // application.run_benchmark_for_20_million_edges().unwrap();
}
