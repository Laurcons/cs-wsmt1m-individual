use std::error::Error;

use axum::{Json, body::Body, http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub struct AppErrorResponse {
  pub status: StatusCode,
  pub code: String,
  pub message: String,
}

#[derive(Serialize)]
struct AppErrorResponseBody {
  pub code: String,
  pub message: String,
}

impl IntoResponse for AppErrorResponse {
  fn into_response(self) -> axum::response::Response<Body> {
    (
      self.status,
      Json(AppErrorResponseBody {
        code: self.code,
        message: self.message,
      }),
    )
      .into_response()
  }
}

pub fn map_internal_error(err: Box<dyn Error>) -> AppErrorResponse {
  println!("{}", err.to_string());
  AppErrorResponse {
    status: StatusCode::INTERNAL_SERVER_ERROR,
    code: "InternalServerError".to_string(),
    message: "Something went wrong".to_string(),
  }
}

pub fn invalid_id() -> AppErrorResponse {
  AppErrorResponse {
    status: StatusCode::UNPROCESSABLE_ENTITY,
    code: "InvalidId".to_string(),
    message: "The id provided is invalid".to_string(),
  }
}
