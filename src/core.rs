use std::io;
use std::io::Write;
use crossterm::ExecutableCommand;
use rusqlite::Connection;
use crate::tui::metadata_ui;
use crate::db_func::connect_to_db::connect_to_database;

fn interactive_shell() {
    println!("Enter SQL commands (type 'exit' to quit, 'help' for special commands):");
    let mut connection: Option<Connection> = None;

    loop {
        print!("sqliteCLI> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();
        let mut parts = trimmed_command.split_whitespace();

        match parts.next() {
            Some("exit") => {
                break;
            }
            Some("help") => {
                print_help();
            }
            Some("browse") => {
                if let Some(ref conn) = connection {
                    metadata_ui::run_tui(conn);
                } else {
                    println!("You need to connect to a database first.");
                }
            }
            Some("connect") => {
                // Extract the database path from the command
                if let Some(db_path) = parts.next() {
                    match connect_to_database(db_path) {
                        Ok(new_connection) => {
                            println!("Connected to the database at {}", db_path);
                            connection = Some(new_connection);
                        }
                        Err(err) => {
                            eprintln!("Error connecting to the database: {:?}", err);
                        }
                    }
                } else {
                    println!("Usage: connect <database_path>");
                }
            }
            _ => {
                // Handle other SQL commands here
                if let Some(ref conn) = connection {
                    match conn.execute(trimmed_command, []) {
                        Ok(_) => {
                            println!("Query executed successfully.");
                        }
                        Err(err) => {
                            eprintln!("Error executing query: {:?}", err);
                        }
                    }
                } else {
                    println!("You need to connect to a database first.");
                }
            }
        }
    }
}

fn connect_to_database(p0: &str) -> Result<Connection, rusqlite::Error> {
    todo!()
}


fn print_help() {
    println!("Special Commands:");
    println!("  exit  : Exit the CLI");
    println!("  help  : Print this help message");
    println!("  BROWSE : Launch the TUI to browse database tables and columns");

    // Add more special commands or instructions here as needed
}

