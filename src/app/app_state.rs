use actix::prelude::Addr;
use crate::repository::Repository;
use crate::configuration::AppConfiguration;

pub struct AppState {
    pub repository: Addr<Repository>,
    pub app_configuration: AppConfiguration,
}