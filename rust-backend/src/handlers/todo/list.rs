use axum::{
  Json,
  extract::{Query, State},
  response::ErrorResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
  app_state::AppState, database::models::Todo, services::todo_service::ListOptions,
  utils::errors::map_internal_error,
};

#[derive(Deserialize)]
pub struct ListQuery {
  pub is_done: Option<String>,
}

#[derive(Serialize)]
pub struct ListResponse {
  pub todos: Vec<Todo>,
}

#[axum::debug_handler]
pub async fn list(
  query: Query<ListQuery>,
  state: State<AppState>,
) -> Result<Json<ListResponse>, ErrorResponse> {
  let options = ListOptions {
    is_done: match &query.is_done {
      Some(str) => Some(str.as_str().eq("true")),
      None => None,
    },
  };
  Ok(Json(ListResponse {
    todos: state
      .todo_service
      .list(options)
      .map_err(map_internal_error)?,
  }))
}
