pub mod app_state;

use app_state::AppState;
use crate::configuration::AppConfiguration;
use actix_cors::{Cors, CorsFactory};
use actix_web::{web, middleware::Logger, HttpServer, App, web::Data, http::header::{AUTHORIZATION, CONTENT_TYPE}, HttpResponse};
use actix::{Addr, SyncArbiter};
use crate::repository::Repository;

pub fn start(config: AppConfiguration)  {
    let app_url = format!("127.0.0.1:{}", config.app_port);
    HttpServer::new(move || {
        let cors = get_cors(&config);
        let database_address = SyncArbiter::start(num_cpus::get(), move || Repository::new(config.databaseUrl().clone()));
        let state = AppState {
            app_configuration: config.clone(),
            repository: database_address,
        };
        App::new()
            .data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routing_configuration)
        })
        .bind(app_url)
        .unwrap()
        .run();
}

fn get_cors(config: &AppConfiguration)-> CorsFactory {
    match &config.frontend_url {
        Some(frontend_url) => Cors::new()
            .allowed_origin(&frontend_url)
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            .max_age(3600)
            .finish(),
        None => Cors::new()
            .allowed_origin("*")
            .send_wildcard()
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
            .max_age(3600)
            .finish(),
    }
}

fn routing_configuration(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to( || HttpResponse::Ok()));
}
