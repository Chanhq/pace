use pace::application::Application;

fn main(){
    let application = Application::new();

    application.run_test_on_randomly_generated_graphs().unwrap();
}
