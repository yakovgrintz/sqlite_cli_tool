mod db_func;
mod tui;
mod core;

use clap::{Arg, Command};
use std::io::{Write};
use crate::core::core_shell::interactive_shell;


fn main() {
    let app = Command::new("SQLite CLI Tool")
        .version("0.1.0")
        .author("Your Name")
        .about("Interacts with SQLite databases")
        .arg(Arg::new("DATABASE")
            .help("Sets the SQLite database file to use")
            .required(false)
            .index(1));

    interactive_shell();
}


