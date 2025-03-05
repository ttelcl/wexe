#![allow(dead_code)]

use wexe::console_colors::*;

pub struct CommandHelp {
    pub command: String,
    pub synopsis: String,
    pub description: String,
    pub options: Vec<String>,
}

impl CommandHelp {
    pub fn print(&self) {
        println!("{}{rst}", self.synopsis);
        println!("    {}{rst}", self.description);
        for opt in self.options.iter() {
            println!("    {}{rst}", opt);
        }
    }
}

fn init_help() -> Vec<CommandHelp> {
    let mut help = Vec::new();
    help.push(
        CommandHelp {
            command: "/help".into(),
            synopsis: format!("{fg_o}/help{rst}"),
            description: "Show help for all commands".into(),
            options: Vec::new(),
        }
    );
    help.push(
        CommandHelp {
            command: "/list".into(),
            synopsis: format!("{fg_o}/list{rst} [{fg_g}-m {fg_c}{stl_i}filter{rst}]"),
            description: "List all configured applications".into(),
            options: vec![
                format!("{fg_g}-m {fg_c}{stl_i}filter{rst}       If given, only list applications with the {fg_c}{stl_i}filter{rst} string in their name."),
            ],
        }
    );
    help
}

pub struct HelpCentral {
    commands: Vec<CommandHelp>,
}

impl HelpCentral {
    pub fn new() -> HelpCentral {
        let help = init_help();
        HelpCentral {
            commands: help,
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

    pub fn print_all_help(&self) {
        for help in self.commands.iter() {
            help.print();
        }
    }
}
