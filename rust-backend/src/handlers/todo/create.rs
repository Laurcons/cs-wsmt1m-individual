use axum::{
  extract::{self, State},
  http::StatusCode,
  response::{ErrorResponse, IntoResponse},
};
use axum_valid::Valid;
use serde::Deserialize;
use validator::Validate;

use crate::{
  app_state::AppState, services::todo_service::CreateTodo, utils::errors::map_internal_error,
};

#[derive(Validate, Deserialize)]
pub struct CreateBody {
  #[validate(length(max = 512, message = "Your title is too long!"))]
  title: String,
}

#[axum::debug_handler]
pub async fn create(
  state: State<AppState>,
  Valid(extract::Json(body)): Valid<extract::Json<CreateBody>>,
) -> Result<impl IntoResponse, ErrorResponse> {
  state
    .todo_service
    .create(CreateTodo { title: body.title })
    .map_err(map_internal_error)?;
  Ok(StatusCode::NO_CONTENT)
}
