use rusqlite::Connection;
use rusqlite::{Connection, Error, Result};
use std::fs;
use std::path::Path;

pub(crate) fn connect_to_database(db_path: &str) -> Result<Connection> {
    let path = Path::new(db_path);

    if path.exists() {
        match Connection::open(path) {
            Ok(connection) => {
                println!("Connection established to an existing database.");
                Ok(connection)
            }
            Err(err) => Err(err),
        }
    } else {
        Err(Error::SqliteFailure(
            rusqlite::ffi::Error::new(101, "Database file does not exist"), // You can choose your custom error code and message
            Some("Database file does not exist"),
        ))
    }
}
pub(crate) fn create_database(db_path: &str) -> Result<Connection>{
    todo!(Create an option to create a database
If it already exists, return an error and print that the file already exists and a database link method should be used)
}