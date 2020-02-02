pub mod app_state;

use app_state::AppState;
use crate::configuration::AppConfiguration;
use actix_cors::{Cors, CorsFactory};
use actix_web::{
    HttpServer,
    App,
    web::Data,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};

pub fn start(config: AppConfiguration)  {
    let app_url = format!("127.0.0.1:{}", config.app_port);
    HttpServer::new(move || {
        let cors = get_cors(&config);
        let state = AppState {};
        App::new()
            .data(Data::new(state))
            // .wrap(Logger::default())
            .wrap(cors)
            // .configure(routes)
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