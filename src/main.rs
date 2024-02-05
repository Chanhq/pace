use pace::application::Application;


fn main(){
    let application = Application::new();

    application.run_tests_with_same_edges().unwrap();
    // application.run_tests_with_same_nodes().unwrap();
    // application.run_small_tests().unwrap();
}
