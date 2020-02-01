#[macro_use]
extern crate envconfig_derive;

mod configuration;
mod app;

use dotenv;
use envconfig::Envconfig;
use std::{env};

fn main() {
    dotenv::dotenv().ok();
    let app_configuration = match configuration::AppConfiguration::init() {
        Ok(configuration) => configuration,
        Err(cause) => panic!("Error initializing Application Configuration: {:?}", cause)
    };
    configuration::setup_logger(&app_configuration);

    let sys = actix::System::new("app");
    app::start(&app_configuration);
    let _ = sys.run();
}
