use crate::{
  app_config::AppConfig, database::connect_to_database, services::todo_service::TodoService,
};

#[derive(Clone)]
pub struct AppState {
  // pub mysql: Pool<ConnectionManager<MysqlConnection>>,
  pub config: AppConfig,
  pub todo_service: TodoService,
}

impl AppState {
  pub fn new(config: &AppConfig) -> AppState {
    let mysql = connect_to_database(config);
    AppState {
      config: config.clone(),
      todo_service: TodoService::new(mysql),
    }
  }
}
