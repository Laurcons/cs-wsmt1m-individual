use crate::{database::connect_to_database, services::todo_service::TodoService};

#[derive(Clone)]
pub struct AppState {
  // pub mysql: Pool<ConnectionManager<MysqlConnection>>,
  pub todo_service: TodoService,
}

impl AppState {
  pub fn new() -> AppState {
    let mysql = connect_to_database();
    AppState {
      todo_service: TodoService::new(mysql),
    }
  }
}
