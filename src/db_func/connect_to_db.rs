
use rusqlite::{Connection, Error, Result};
use std::path::Path;

pub(crate) fn connect_to_database(db_path: &str) -> Result<Connection> {
    let path = Path::new(db_path);

    if path.exists() {
        match Connection::open(path) {
            Ok(connection) => {
                Ok(connection)
            }
            Err(err) => Err(err),
        }
    } else {
        Err(Error::SqliteFailure(
            rusqlite::ffi::Error::new(101), // You can choose your custom error code and message
            Some("Database file does not exist".parse().unwrap()),
        ))
    }
}
pub(crate) fn create_database(db_path: &str) -> Result<Connection>{
    let path = Path::new(db_path);
    if !path.exists() {
        match Connection::open(path) {
            Ok(connection) => {
                println!("Created the database named: {} at the path: {:?}", path.file_name().unwrap().to_str().unwrap(), path);
                Ok(connection)
            }
            Err(err) => Err(err),
        }
    } else {
        Err(Error::SqliteFailure(
            rusqlite::ffi::Error::new(102), // Custom error code indicating the database already exists
            Some("Database file already exists".to_string()),
        ))
    }
    }
