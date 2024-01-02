mod db_func;
mod tui;
mod core;

use clap::{Arg, Command};
use std::io::{Write};
use crate::core::cli;
use crate::core::commands::handle_commands;
use crate::db_func::db_manager::DbManager;use std::sync::{Arc, Mutex};


fn main() {

    let matches = cli::build_cli().get_matches();
    let db_manager = Arc::new(Mutex::new(DbManager::new()));
    handle_commands::handle_command(matches,db_manager);


}

