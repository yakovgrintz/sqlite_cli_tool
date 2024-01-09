
use clap::{ Parser, Subcommand};


#[derive(Parser)]
#[clap(author = "Yakov Grintz", version = "0.1.0", about = "Interacts with SQLite databases")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Show head of table
    Show {
        /// Name of the table to show
        #[arg()]
        table_name: String,
    },
    /// Browse the database
    Browse,
    /// Exit from app
    Exit,
    /// Show help UI
    HelpUi,
    /// Connect to DB
    Connect {
        #[clap()]
        db_path: String,
    },
}

// pub fn build_cli() -> Command {
//     Command::new("SQLite CLI")
//         .version("0.1.0")
//         .author("Yakov Grintz")
//         .about("Interacts with SQLite databases")
//         .subcommand(Command::new("show")
//             .about("show head of table")
//             .arg(Arg::new("table_name")
//                 .help("name of table to show")
//                 .required(true)
//                 .index(1)))
//         .subcommand(Command::new("browse")
//             .about("Browse the database"))
//         .subcommand(Command::new("exit").about("exit from app")).subcommand(Command::new("help_ui").about("show help ui"))
//         .subcommand(Command::new("connect").about("connect to db").arg(
//             Arg::new("db_path")
//                 .help("path to db that you want to connect")
//                 .required(true)
//                 .index(1)
//         ))
//
//
// }
// pub(crate) fn build_cli(){
//     let db_manager = Arc::new(Mutex::new(DbManager::new()));
//     let mut input = String::new();
//     loop {
//         print!("sqlite_cli> ");  // Your prompt to the user
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).unwrap();  // Read a line of input
//
//         if input.trim() == "exit" {  // Allow an 'exit' command to break the loop
//             break;
//         }
//
//         // Split the input into arguments and parse as commands
//         let args = shell_words::split(&input).unwrap_or_else(|_| Vec::new());
//         let cli = Cli::parse_from(args.iter().map(|s| s.as_str()));
//
//         // Handle the command
//         handle_command(&cli, Arc::clone(&db_manager));
//
//     }}
