
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

    let matches = app.get_matches();

    if let Some(db_path) = matches.get_one::<String>("DATABASE") {
        println!("Database path is: {}", db_path);
        if let Err(e) = connect_to_database(db_path) {
            println!("Failed to connect to database: {}", e);
        }
    } else {
        println!("No database path provided. Please specify the path to the database file to start.");
        println!("Usage: sqlite_cli <DATABASE>");
    }
}

fn connect_to_database(db_path: &str) -> Result<()> {
    let path = Path::new(db_path);
    let connection = Connection::open(path)?;

    if path.exists() {
        println!("Connection established to existing database.");
    }
    /*else {
        println!("New database has been created and connected.");
    }*/

    interactive_shell(connection)
}

fn interactive_shell(mut connection: Connection) -> Result<()> {
    println!("Enter SQL commands (type 'exit' to quit, 'help' for special commands):");
    loop {
        print!("sqliteCLI> ");
        io::stdout().flush().unwrap(); // Make sure 'sqlite>' prompt appears immediately

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();
        match trimmed_command {
            "exit" => break,
            "help" => print_help(),
            "BROWSE" => metadata_ui::run_tui(&connection),
            _ => {
                if trimmed_command.to_lowercase().starts_with("select") {
                    // Handle SELECT queries which expect results
                    let mut stmt = connection.prepare(trimmed_command)?;
                    let rows = stmt.query_map([], |row| {
                        // Now you define how to convert each row into a desired format.
                        // This is an example assuming all results are a single string column.
                        row.get::<_, String>(0) // Adjust the column index and type according to your needs
                    })?;

                    for result in rows {
                        match result {
                            Ok(value) => println!("{}", value),
                            Err(e) => println!("Row error: {}", e),
                        }
                    }
                } else {
                    // Execute other SQL commands
                    match connection.execute(trimmed_command, []) {
                        Ok(changes) => println!("Executed successfully, {} row(s) affected.", changes),
                        Err(e) => println!("Error executing command: {}", e),
                    }
                }
            }
        }
    }

    Ok(())
}


fn print_help() {
    println!("Special Commands:");
    println!("  exit  : Exit the CLI");
    println!("  help  : Print this help message");
    println!("  BROWSE : Launch the TUI to browse database tables and columns");

    // Add more special commands or instructions here as needed
}
