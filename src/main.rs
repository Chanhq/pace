use pace::application::Application;


fn main(){
    let application = Application::new();

    // application.run_on_specific_case(500, 1000).unwrap();
    // application.run_tests_on_tiny_test_set().unwrap();
    application.run_small_tests().unwrap();
}
