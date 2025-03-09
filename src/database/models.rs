use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::counter)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Counter {
  pub id: i32,
  pub value: Option<i32>,
}
