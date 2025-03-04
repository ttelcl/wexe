#![allow(dead_code)]

use wexe::console_colors::*;

pub struct CommandHelp {
    pub command: String,
    pub synopsis: String,
    pub description: String,
    pub options: Vec<String>,
}

impl CommandHelp {
    pub fn new(
        command: &str,
        synopsis: &str,
        description: &str,
        options: &Vec<&str>,
    ) -> CommandHelp {
        let mut command_help = CommandHelp {
            command: command.to_string(),
            synopsis: synopsis.to_string(),
            description: description.to_string(),
            options: Vec::new(),
        };
        for opt in options.iter() {
            command_help.add_option(opt);
        }
        command_help
    }

    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }

    pub fn print(&self) {
        println!("{}{rst}", self.synopsis);
        println!("    {}{rst}", self.description);
        for opt in self.options.iter() {
            println!("    {}{rst}", opt);
        }
    }
}

pub struct HelpCentral {
    commands: Vec<CommandHelp>,
}

impl HelpCentral {
    pub fn new() -> HelpCentral {
        HelpCentral {
            commands: Vec::new(),
        }
    }

    pub fn add_help(&mut self, command: CommandHelp) {
        self.commands.push(command);
    }

    pub fn get_help(&self, command: &str) -> Option<&CommandHelp> {
        for cmd in self.commands.iter() {
            if cmd.command == command {
                return Some(cmd);
            }
        }
        None
    }

    pub fn print_help_for(&self, command: &str) {
        match self.get_help(command) {
            Some(help) => {
                help.print();
            }
            None => {
                println!(
                    "{fg_o}No help available for command{rst} '{fg_y}{}{rst}'.",
                    command
                );
            }
        }
    }
}
