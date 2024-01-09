    use crate::db_func::db_manager::DbManager;

    pub fn execute(db_path: &str, db_manager: &mut DbManager) -> Result<(), String> {
        // Attempt to acquire the lock and handle the error if it fails

        // Attempt to connect to the database and handle connection errors
        db_manager.connect(db_path).map_err(|e| format!("Failed to connect to database: {}", e))?;

        println!("Connected to the database at {}", db_path);

        // If everything went well, return Ok
        Ok(())
    }

