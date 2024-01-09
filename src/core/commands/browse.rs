use rusqlite::Error;
use crate::db_func::db_manager::DbManager;
use crate::ui::metadata_ui::run_tui;

pub(crate) fn execute(db_manager: &DbManager) -> rusqlite::Result<()> {
    if let Some(conn_arc) = &db_manager.conn {
        let lock_result = conn_arc.try_lock();
        match lock_result {
            Ok(conn_guard) => {
                run_tui(&*conn_guard); // pass the dereferenced Connection
                Ok(())
            },
            Err(_) => {
                // Handle the error case where the lock could not be acquired
                Err(Error::SqliteFailure(
                    rusqlite::ffi::Error::new(101), // You can choose your custom error code and message
                    Some("please wait to another thread".parse().unwrap()),
                ))  // Or choose an appropriate error
            }
        }
    } else {
        // Handle the case where there is no database connection established
        Err(Error::SqliteFailure(
                        rusqlite::ffi::Error::new(101), // You can choose your custom error code and message
                        Some("you need connect to db first".parse().unwrap()),
                    ))
    }
}