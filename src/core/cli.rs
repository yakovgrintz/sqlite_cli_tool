use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("SQLite CLI")
        .version("0.1.0")
        .author("Yakov Grintz")
        .about("Interacts with SQLite databases")
        .subcommand(Command::new("show")
            .about("show head of table")
            .arg(Arg::new("table_name")
                .help("name of table to show")
                .required(true)
                .index(1)))
        .subcommand(Command::new("browse")
            .about("Browse the database"))
        .subcommand(Command::new("exit").about("exit from app")).subcommand(Command::new("help_ui").about("show help ui"))
        .subcommand(Command::new("connect").about("connect to db").arg(
            Arg::new("db_path")
                .help("path to db that you want to connect")
                .required(true)
                .index(1)
        ))


}
