use pace::application::Application;


fn main(){
    let application = Application::new();

    application.run_tests_with_same_edges_and_free_nodes().unwrap();
}
