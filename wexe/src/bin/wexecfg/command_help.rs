use std::error::Error;
use std::process::ExitCode;

use crate::args_buffer::ArgumentsBuffer;
use crate::commands::{Command, CommandCollection};

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
        commands.print_all_help()
    }
}
