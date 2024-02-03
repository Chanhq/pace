use pace::application::Application;

fn main(){
    let application = Application::new();

    application.run_tests_on_tiny_test_set().unwrap();
}
