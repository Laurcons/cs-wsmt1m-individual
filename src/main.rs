mod database;
use std::time::Duration;

use database::schema::counter::value;
use database::{connect_to_database, models::Counter, schema::counter};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MysqlConnection};

use axum::{
  Router,
  extract::State,
  http::StatusCode,
  routing::{get, post},
};

#[derive(Clone)]
struct AppState {
  mysql: Pool<ConnectionManager<MysqlConnection>>,
}

fn map_internal_error<TE>(err: TE) -> (StatusCode, String)
where
  TE: std::error::Error,
{
  println!("{}", err.to_string());
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    "Something went wrong".to_string(),
  )
}

async fn read(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
  let mut db = state.mysql.get().map_err(map_internal_error)?;
  let counter = db
    .transaction(|conn| {
      counter::table
        .select(Counter::as_select())
        .load::<Counter>(conn)
    })
    .map_err(map_internal_error)
    .unwrap();
  let counter = counter
    .get(0)
    .ok_or((
      StatusCode::INTERNAL_SERVER_ERROR,
      "No counter row".to_string(),
    ))?
    .value
    .ok_or((
      StatusCode::INTERNAL_SERVER_ERROR,
      "No counter value".to_string(),
    ))?;
  Ok(format!("{}\n", counter))
}

async fn increment(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
  let mut db = state.mysql.get().map_err(map_internal_error)?;
  db.transaction(|conn| {
    diesel::update(counter::dsl::counter)
      .set(value.eq(value + 1))
      .execute(conn)
  })
  .map_err(map_internal_error)?;
  // std::thread::sleep(Duration::new(5, 0));
  // tokio::time::sleep(Duration::new(5, 0)).await;
  Ok(format!("Incremented!\n"))
}

#[tokio::main]
async fn main() {
  let mysql_pool = connect_to_database();

  let state = AppState { mysql: mysql_pool };

  // build our application with a single route
  let app = Router::new()
    .route("/", get(read))
    .route("/inc", post(increment))
    .with_state(state);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
