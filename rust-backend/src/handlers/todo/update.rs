use axum::{
  extract::{self, Path, State},
  http::StatusCode,
  response::{ErrorResponse, IntoResponse},
};
use axum_valid::Valid;
use serde::Deserialize;
use validator::Validate;

use crate::{
  app_state::AppState,
  services::todo_service::UpdateTodo,
  utils::errors::{invalid_id, map_internal_error},
};

#[derive(Validate, Deserialize)]
pub struct UpdateBody {
  #[validate(length(max = 14, message = "Your title is too long!"))]
  title: Option<String>,
  is_done: Option<bool>,
}

#[axum::debug_handler]
pub async fn update(
  state: State<AppState>,
  Path(id): Path<String>,
  Valid(extract::Json(body)): Valid<extract::Json<UpdateBody>>,
) -> Result<impl IntoResponse, ErrorResponse> {
  let id = id.parse::<i32>().map_err(|_| invalid_id())?;
  state
    .todo_service
    .update(
      id,
      UpdateTodo {
        title: body.title,
        is_done: body.is_done,
      },
    )
    .map_err(map_internal_error)?;
  Ok(StatusCode::NO_CONTENT)
}
