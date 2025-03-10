use std::env;

use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppConfig {
  pub database_url: String,
  pub port: u16,
}

impl AppConfig {
  pub fn new() -> AppConfig {
    match dotenv() {
      Ok(_) => {}
      Err(_) => println!("Could not read .env, defaulting to envvars"),
    }
    AppConfig {
      database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
      port: env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a number"),
    }
  }
}
