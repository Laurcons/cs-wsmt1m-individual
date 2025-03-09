use diesel::{
  MysqlConnection,
  r2d2::{ConnectionManager, Pool},
};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
