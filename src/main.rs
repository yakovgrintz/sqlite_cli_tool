mod db_func;
mod tui;
mod core;

use clap::{Arg, Command, Parser};
use std::io::{Write};
use crate::core::cli;
use crate::core::commands::handle_commands;
use crate::db_func::db_manager::DbManager;use std::sync::{Arc, Mutex};
use crate::core::cli::Cli;


fn main() {

    let db_manager = Arc::new(Mutex::new(DbManager::new()));
    let cli = Cli::parse();
    handle_commands::handle_command(&cli,db_manager);


}

