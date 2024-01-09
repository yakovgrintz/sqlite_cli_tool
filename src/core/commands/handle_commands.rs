use crate::core::cli::{Cli, Commands};
use crate::core::commands::{browse, connect, exit, help_ui, show};
use crate::db_func::db_manager::DbManager;

// Re-exporting each command module's execute function


// A function to handle commands based on matches
pub fn handle_command(cli: &Cli, db_manager: &mut DbManager) {
    match &cli.command {
        Commands::Show { table_name } => {
            show::execute(table_name, db_manager).expect("TODO: panic message");
        },
        Commands::Browse => {
            browse::execute(db_manager).expect("TODO: panic message");
        },
        Commands::Exit => {
            exit::execute();
        },
        Commands::HelpUi => {
            help_ui::execute();
        },
        Commands::Connect { db_path } => {
            connect::execute(db_path, db_manager).expect("TODO: panic message");
        },
        // ... handle other commands as needed
    }
}
