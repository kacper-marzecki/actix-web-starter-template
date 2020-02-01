extern crate envconfig;

use envconfig::Envconfig;
use std::env;


#[allow(dead_code)]
#[derive(Envconfig)]
#[derive(Debug)]
pub struct AppConfiguration {
    #[envconfig(from = "DB_HOST", default = "127.0.0.1")]
    pub database_host: String,
    #[envconfig(from = "DB_HOST", default = "5432")]
    pub database_port: String,
    #[envconfig(from = "DB_HOST", default = "")]
    pub database_url: String,
    #[envconfig(from = "RUST_LOG", default = "app=debug,actix_web=info")]
    pub env_log_configuration: String,
}

#[allow(dead_code)]
impl AppConfiguration {
    fn get_database_url(self: &AppConfiguration) -> String {
        return if self.database_url.is_empty() {
            format!("{}:{}", self.database_host.clone(), &self.database_port.clone()) 
        } else {
            self.database_url.clone()
        }
    }
}

pub fn setup_logger(configuration: &AppConfiguration){
    env::set_var("RUST_LOG", configuration.env_log_configuration.clone());
    env_logger::init();
}