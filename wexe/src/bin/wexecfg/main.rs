use std::error::Error;
use std::process::ExitCode;

use wexe::console_colors::*;

use args_buffer::ArgumentsBuffer;
use commands::CommandCollection;

mod args_buffer;
mod command_fix;
mod command_help;
mod command_install;
mod command_list;
mod command_wrap;
mod commands;
mod help_central;
mod wexe_repository;

fn setup_commands() -> CommandCollection {
    let mut commands = CommandCollection::new();
    commands.add_command(Box::new(command_help::HelpCommand::new()));
    commands.add_command(Box::new(command_list::ListCommand::new()));
    commands.add_command(Box::new(command_install::InstallCommand::new()));
    commands.add_command(Box::new(command_wrap::WrapCommand::new()));
    commands.add_command(Box::new(command_fix::FixCommand::new()));
    commands
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    println!("{fg_g}{stl_i}WEXE executable wrapper - Configuration Utility{rst}.");

    let commands = setup_commands();
    let mut arguments = ArgumentsBuffer::new(std::env::args().skip(1).collect());

    match arguments.peek() {
        Some(name) => {
            let name = name.to_string();
            let command = commands.get_command(&name);
            arguments.skip(1);
            match command {
                Some(command) => {
                    command.execute(&mut arguments, &commands)
                },
                None => {
                    eprintln!(
                        "{fg_r}Unknown command: {fg_o}{:}{rst}.",
                        name
                    );
                    commands.print_all_help()
                }
                
            }
        }
        None => {
            eprintln!("{fg_r}No command specified{rst}.");
            commands.print_all_help()
        }
    }
}
