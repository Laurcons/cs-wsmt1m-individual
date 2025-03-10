use axum::{Json, extract::State, response::ErrorResponse};
use serde::Serialize;

use crate::{
  app_state::AppState,
  database::models::Todo,
  utils::errors::map_internal_error,
};

#[derive(Serialize)]
pub struct ListResponse {
  todos: Vec<Todo>,
}

#[axum::debug_handler]
pub async fn list(state: State<AppState>) -> Result<Json<ListResponse>, ErrorResponse> {
  Ok(Json(ListResponse {
    todos: state.todo_service.list().map_err(map_internal_error)?,
  }))
}
