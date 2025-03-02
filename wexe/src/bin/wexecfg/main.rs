use std::error::Error;
use std::process::ExitCode;

use wexe::console_colors::*;

use commands::CommandCollection;

mod command_help;
mod command_list;
mod commands;

fn setup_commands() -> CommandCollection {
    let mut commands = CommandCollection::new();
    commands.add_command(Box::new(command_help::HelpCommand::new()));
    commands.add_command(Box::new(command_list::ListCommand::new()));
    commands
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let commands = setup_commands();

    println!("{fg_g}{stl_i}WEXE executable wrapper - Configuration Utility{rst}.");
    println!("Not yet implemented.");
    commands.print_help()
}
