#[macro_use]
extern crate envconfig_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

mod app;
mod common;
mod configuration;
mod model;
mod repository;
mod service;

use dotenv;
use envconfig::Envconfig;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    let app_configuration = match configuration::AppConfiguration::init() {
        Ok(configuration) => configuration,
        Err(cause) => panic!("Error initializing Application Configuration: {:?}", cause),
    };
    configuration::setup_logger(&app_configuration);

    let sys = actix::System::new("app");
    app::start(app_configuration);
    let _ = sys.run();
}
