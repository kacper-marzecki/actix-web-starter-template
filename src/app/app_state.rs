use actix::prelude::Addr;
use crate::repository::Repository;
use crate::configuration::AppConfiguration;

pub struct AppState {
    repository: Addr<Repository>,
    app_configuration: AppConfiguration,
}