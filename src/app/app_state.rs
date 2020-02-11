use crate::configuration::AppConfiguration;
use crate::repository::Repository;
use actix::prelude::Addr;

pub struct AppState {
    pub repository: Addr<Repository>,
    pub app_configuration: AppConfiguration,
}
