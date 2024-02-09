use pace::application::Application;


fn main(){
    let application = Application::new();

    application.run_tests_with_same_edges_and_free_nodes().unwrap();
    application.run_on_specific_case(500, 20).unwrap();
    application.run_tests_on_tiny_test_set().unwrap();
    application.run_tests_with_same_edges().unwrap();
    application.run_tests_with_same_nodes().unwrap();
}
