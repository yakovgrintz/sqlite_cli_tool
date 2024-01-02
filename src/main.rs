mod core;

use clap::{Arg, Command};
use rusqlite::{Connection, Result};
use std::io::{self, Write};
use std::path::Path;
use crate::tui::metadata_ui;

mod db_func {
    pub(crate) mod metadata_func;
}
mod tui{
    pub(crate) mod metadata_ui;
}
fn main() {
    let app = Command::new("SQLite CLI Tool")
        .version("0.1.0")
        .author("Your Name")
        .about("Interacts with SQLite databases")
        .arg(Arg::new("DATABASE")
            .help("Sets the SQLite database file to use")
            .required(false)
            .index(1));

    interactive_shell().expect("TODO: panic message");
}

fn interactive_shell() {
    println!("Enter SQL commands (type 'exit' to quit, 'help' for special commands):");
    let mut connection:Connection;
    loop {
        print!("sqliteCLI> ");
        io::stdout().flush().unwrap(); // Make sure 'sqlite>' prompt appears immediately

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();
        match trimmed_command {
            //"connect"=>,
            "exit" => break,
            "help" => print_help(),
            "browse" => metadata_ui::run_tui(&connection),
            _ => {

                }
            }
        }
    }





fn print_help() {
    println!("Special Commands:");
    println!("  exit  : Exit the CLI");
    println!("  help  : Print this help message");
    println!("  BROWSE : Launch the TUI to browse database tables and columns");

    // Add more special commands or instructions here as needed
}
