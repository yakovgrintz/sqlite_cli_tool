
use crate::db_func::db_manager::DbManager;
use rusqlite::Error;
use crate::ui::show_data::show;

pub(crate) fn execute(table_name: &str, db_manager: &DbManager) -> rusqlite::Result<()> {
    if let Some(conn_arc) = &db_manager.conn {
        let conn_guard = conn_arc.lock().map_err(|_| {
            // Handle lock errors by returning a custom error
            Error::SqliteFailure(
                rusqlite::ffi::Error::new(101),
                Some("Failed to acquire a lock on the database connection. It might be poisoned due to a previous panic.".into())
            )
        })?;

        // Use the show function to display the first 5 rows of the table
        show(&*conn_guard, table_name)
    } else {
        // Return an error if no connection is established
        Err(Error::SqliteFailure(
            rusqlite::ffi::Error::new(102),
            Some("No database connection established".into())
        ))
    }
}