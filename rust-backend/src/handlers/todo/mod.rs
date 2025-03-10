use axum::{
  Router,
  routing::{delete, get, patch, post},
};
use create::create;
use delete::delete as deleteFn;
use list::list;
use update::update;

use crate::app_state::AppState;

mod create;
mod delete;
mod list;
mod update;

pub fn todo_router() -> Router<AppState> {
  Router::new()
    .route("/", get(list))
    .route("/", post(create))
    .route("/{id}", patch(update))
    .route("/{id}", delete(deleteFn))
}
