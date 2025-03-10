
use diesel::{
  MysqlConnection,
  r2d2::{ConnectionManager, Pool},
};

use crate::app_config::AppConfig;

pub mod models;
pub mod schema;

pub fn connect_to_database(config: &AppConfig) -> Pool<ConnectionManager<MysqlConnection>> {
  let database_url = config.database_url.clone();

  println!("Connecting to Mysql...");

  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  // Refer to the `r2d2` documentation for more methods to use
  // when building a connection pool
  Pool::builder()
    .test_on_check_out(true)
    .build(manager)
    .expect("Could not build connection pool")
}
