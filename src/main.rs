mod db_func;
mod ui;
mod core;

use std::io;
use clap::{ Parser};
use std::io::{Write};


use crate::db_func::db_manager::DbManager;
use crate::core::cli::Cli;
use crate::core::commands::handle_commands::handle_command;


fn main() -> Result<(), String> {

    let mut db_manager = DbManager::new();
    loop {
        print!("sqlite_cli> ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        } else if input == "exit" || input == "quit" {
            break; // Exit the REPL loop
        }
        let args = shlex::split(input).ok_or("error: Invalid quoting")?;
        println!("{:?}", args);
        let mut clap_args = vec!["sqlite_cli".to_string()];
        clap_args.extend(args.iter().map(|s| s.to_string()));
        let cli = Cli::try_parse_from(clap_args).map_err(|e| e.to_string())?;

        handle_command(&cli, &mut db_manager);
    }

    Ok(())

}
// let cli = Cli::parse();
// handle_commands::handle_command(&cli,db_manager);
