use axum::{extract::Request, middleware::Next, response::Response};
use chrono::Local;

pub async fn log_request(request: Request, next: Next) -> Response {
  let method = request.method().clone();
  let path = request.uri().clone();
  let response = next.run(request).await;
  let status = response.status();
  let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
  println!("[{}] {} {} -> {:?}", current_time, method, path, status);
  response
}
