use std::env;

use diesel::{
  MysqlConnection,
  r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn connect_to_database() -> Pool<ConnectionManager<MysqlConnection>> {
  dotenv().expect("Could not read env");

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  println!("Connecting to Mysql...");

  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  // Refer to the `r2d2` documentation for more methods to use
  // when building a connection pool
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}
