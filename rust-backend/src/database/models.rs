use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::database::schema::todos)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub is_done: bool,
}
