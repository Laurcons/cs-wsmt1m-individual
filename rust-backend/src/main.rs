mod database;

use app_config::AppConfig;
use app_state::AppState;

use axum::{Router, http::StatusCode, routing::get};
use handlers::todo::todo_router;
use tower::ServiceBuilder;
use tower_http::{
  cors::{Any, CorsLayer},
  trace::TraceLayer,
};
use utils::middleware::log_middleware::log_request;

mod app_config;
mod app_state;
mod handlers;
mod services;
mod utils;

#[tokio::main]
async fn main() {
  let config = AppConfig::new();
  let state = AppState::new(&config);

  let cors_layer = CorsLayer::new()
    .allow_origin(Any) // Open access to selected route
    .allow_methods(Any)
    .allow_headers(Any);

  let app = Router::new()
    .route(
      "/",
      get(|| async { (StatusCode::OK, format!("All went ok!\n")) }),
    )
    .nest("/todos", todo_router())
    .layer(
      ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(log_request))
        .layer(cors_layer),
    )
    .with_state(state);

  let port = config.port;
  let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
    .await
    .unwrap();
  println!("Listening");
  axum::serve(listener, app).await.unwrap();
}
