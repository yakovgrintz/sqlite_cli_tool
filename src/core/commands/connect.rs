    use std::sync::{Arc, Mutex};
    use crate::db_func::db_manager::DbManager;

    pub fn execute(db_path: &str, db_manager: Arc<Mutex<DbManager>>) {
        let mut db_manager = db_manager.lock().unwrap();
        db_manager.connect(db_path).unwrap(); // handle errors appropriately
        println!("Connected to the database at {}", db_path);
    }

