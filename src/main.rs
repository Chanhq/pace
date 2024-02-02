use pace::{application::Application, error::Error};

fn main() -> Result<(), Error>{
    let application = Application::new();

    application.run()
}
