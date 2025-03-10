use diesel::{
  dsl::{delete, insert_into},
  prelude::*,
  update,
};
use std::error::Error;

use crate::{
  database::{models::Todo, schema},
  utils::mysql_pool::MysqlPool,
};

#[derive(Clone)]
pub struct TodoService {
  mysql: MysqlPool,
}

pub struct CreateTodo {
  pub title: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::todos)]
pub struct UpdateTodo {
  pub title: Option<String>,
  pub is_done: Option<bool>,
}

impl TodoService {
  pub fn new(mysql: MysqlPool) -> TodoService {
    TodoService { mysql }
  }

  pub fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
    use schema::todos::dsl::*;

    let mut conn = self.mysql.get()?;
    let listing = todos.filter(is_done.eq(false)).load::<Todo>(&mut conn)?;
    Ok(listing)
  }

  pub fn create(&self, data: CreateTodo) -> Result<(), Box<dyn Error>> {
    use crate::database::schema::todos::dsl::*;

    let mut conn = self.mysql.get()?;
    insert_into(todos)
      .values((title.eq(data.title), is_done.eq(false)))
      .execute(&mut conn)?;
    Ok(())
  }

  pub fn update(&self, updated_id: i32, data: UpdateTodo) -> Result<(), Box<dyn Error>> {
    use crate::database::schema::todos::dsl::*;

    let mut conn = self.mysql.get()?;
    update(todos)
      .filter(id.eq(updated_id))
      .set(data)
      .execute(&mut conn)?;
    Ok(())
  }

  pub fn delete(&self, deleted_id: i32) -> Result<(), Box<dyn Error>> {
    use crate::database::schema::todos::dsl::*;

    let mut conn = self.mysql.get()?;
    delete(todos).filter(id.eq(deleted_id)).execute(&mut conn)?;
    Ok(())
  }
}
