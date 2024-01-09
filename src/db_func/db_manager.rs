use std::path::Path;
use rusqlite::{Connection, Error};
use std::sync::{Arc, Mutex};

// Define a structure for global mutable shared connection
pub struct DbManager {
    pub conn: Option<Arc<Mutex<Connection>>>,
}

impl DbManager {
    pub fn new() -> Self {
        Self { conn: None }
    }




    pub fn connect(&mut self, db_path: &str) -> rusqlite::Result<()> {
        let path = Path::new(db_path);

        if path.exists() {
            let conn = Connection::open(db_path)?;
            self.conn = Some(Arc::new(Mutex::new(conn)));
            Ok(())
        } else {
            Err(Error::SqliteFailure(
                rusqlite::ffi::Error::new(101), // You can choose your custom error code and message
                Some("Database file does not exist".parse().unwrap()),
            ))
        }


}
    fn create_database(&mut self, db_path: &str) -> rusqlite::Result<()> {
        let path = Path::new(db_path);
        if !path.exists() {
            let conn = Connection::open(db_path)?;
            self.conn = Some(Arc::new(Mutex::new(conn)));
            Ok(())
        } else {
            Err(Error::SqliteFailure(
                rusqlite::ffi::Error::new(102), // Custom error code indicating the database already exists
                Some("Database file already exists".to_string()),
            ))
        }
    }
}

