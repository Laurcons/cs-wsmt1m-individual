use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::{ErrorResponse, IntoResponse},
};

use crate::{
  app_state::AppState,
  utils::errors::{invalid_id, map_internal_error},
};

#[axum::debug_handler]
pub async fn delete(
  state: State<AppState>,
  Path(id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
  let id = id.parse::<i32>().map_err(|_| invalid_id())?;
  state.todo_service.delete(id).map_err(map_internal_error)?;
  Ok(StatusCode::NO_CONTENT)
}
