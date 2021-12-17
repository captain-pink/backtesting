pub mod fake_db {
  use log::{info};
  use diesel::prelude::*;
  use std::any::Any;

  pub struct FakeDb {
    connection: SqliteConnection
  }

  pub struct UserAccount {
    id: u32,
    name: String,
    active: bool,
  }

  impl FakeDb {
    pub fn new() -> Self {
      use std::env;
      use std::path::PathBuf;
    
      const DB_URL: &str = "DB_URL";
      let db_root_path_buf = env::current_dir().unwrap();
      let db_root_path = db_root_path_buf.to_str().unwrap();
      let db_root_path_as_vec: Vec<&str> = vec![db_root_path];
      info!("Project root path found: {}", db_root_path);
    
      let db_relative_path =
          env::var(DB_URL).expect(&format!("No variable found: \"{}\"", DB_URL));
      let db_relative_path_as_vec = db_relative_path.as_str().split("/").collect();
      let db_path_buf: PathBuf = [db_root_path_as_vec, db_relative_path_as_vec]
          .concat()
          .iter()
          .collect();
      let db_path = db_path_buf.to_str().unwrap();
      info!("DB path compiled: {}", db_path);
    
      let connection = SqliteConnection::establish(db_path).unwrap();
      info!("DB connectioni established");

      FakeDb { connection }
    }
  }
}
