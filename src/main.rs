mod database;

use app_state::AppState;

use axum::{Router, http::StatusCode, routing::get};
use handlers::todo::todo_router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utils::middleware::log_middleware::log_request;

mod app_state;
mod handlers;
mod services;
mod utils;

#[tokio::main]
async fn main() {
  let state = AppState::new();

  let app = Router::new()
    .route(
      "/",
      get(|| async { (StatusCode::OK, format!("All went ok!\n")) }),
    )
    .nest("/todos", todo_router())
    .layer(
      ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(log_request)),
    )
    .with_state(state);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Listening");
  axum::serve(listener, app).await.unwrap();
}
