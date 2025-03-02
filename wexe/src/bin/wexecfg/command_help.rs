use std::error::Error;
use std::process::ExitCode;

use crate::args_buffer::ArgumentsBuffer;
use crate::commands::{Command, CommandCollection};

use wexe::console_colors::*;

pub struct HelpCommand {
    names: Vec<&'static str>,
}

impl HelpCommand {
    pub fn new<'a>() -> HelpCommand {
        HelpCommand {
            names: vec!["/help", "/h"],
        }
    }
}

impl Command for HelpCommand {
    fn name(&self) -> &str {
        self.names[0]
    }

    fn name_and_aliases(&self) -> &[&str] {
        self.names.as_ref()
    }

    fn execute(&self, _args: &mut ArgumentsBuffer, commands: &CommandCollection) -> Result<ExitCode, Box<dyn Error>> {
        for cmd in commands.get_commands().iter() {
            cmd.print_help();
        }
        Ok(ExitCode::FAILURE) // soft failure
    }

    fn print_help(&self) -> () {
        println!(r#"{fg_o}/help{rst}\n 
    Show help for all commands"#
        );
    }
}
