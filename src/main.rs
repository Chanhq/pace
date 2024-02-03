use pace::application::Application;

fn main(){
    let application = Application::new();

    application.run_benchmark_for_20_million_edges().unwrap();
    application.run_benchmark_for_1_million_fixed_and_free_nodes().unwrap();
}
