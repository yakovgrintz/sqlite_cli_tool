use std::sync::{Arc, Mutex};
use crate::core::commands::{browse, connect, exit, help_ui, show};
use crate::db_func::db_manager::DbManager;

// Re-exporting each command module's execute function


// A function to handle commands based on matches
pub fn handle_command(matches: clap::ArgMatches,db_manager: Arc<Mutex<DbManager>>) {
    match matches.subcommand() {
        Some(("show", sub_m)) => {
            let table_name = sub_m.value_of("table_name").unwrap();
            show::execute(table_name);
        }
        Some(("browse", _)) => {
            browse::execute();
        }
        Some(("exit", _)) => {
            exit::execute();
        }
        Some(("help_ui", _)) => {
            help_ui::execute();
        }
        Some(("connect", sub_m)) => {
            let db_path = sub_m.value_of("db_path").unwrap();
            connect::execute(db_path,);
        }
        // ... handle other commands as needed
        _ => unreachable!(), // Assuming you always provide a valid subcommand
    }
}
