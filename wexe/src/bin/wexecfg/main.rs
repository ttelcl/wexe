use wexe::console_colors::*;

use commands::CommandCollection;

mod commands;

fn setup_commands() -> CommandCollection {
    #[allow(unused_mut)]
    let mut commands = CommandCollection::new();
    // commands.add_command(Box::new(commands::HelpCommand::new()));
    // commands.add_command(Box::new(commands::ListCommand::new()));
    // commands.add_command(Box::new(commands::SetCommand::new()));
    // commands.add_command(Box::new(commands::UnsetCommand::new()));
    commands
}

fn main() {
    println!("Hello, {fg_g}wexecfg{rst}! ({stl_i}{stl_d}Not yet implemented{rst})");
    #[allow(unused_variables)]
    let commands = setup_commands();
}
