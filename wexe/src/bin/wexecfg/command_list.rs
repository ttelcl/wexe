
use std::{error::Error, process::ExitCode};

use crate::{args_buffer::ArgumentsBuffer, commands::{Command, CommandCollection}};

use wexe::console_colors::*;

pub struct ListCommand {
    names: Vec<&'static str>,
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            names: vec!["/list", "/l"],
        }
    }
}

impl Command for ListCommand {

    fn name(&self) -> &str {
        self.names[0]
    }

    fn name_and_aliases(&self) -> &[&str] {
        self.names.as_ref()
    }

    fn execute(&self, _args: &mut ArgumentsBuffer, _commands: &CommandCollection) -> Result<ExitCode, Box<dyn Error>> {
        panic!("Not yet implemented: ListCommand.execute.");
    }

    fn print_help(&self) -> () {
        println!(r#"{fg_o}/list{rst} [{fg_g}-m {fg_c}{stl_i}filter{rst}]\n 
    List the registered applications."#
        );
    }
}
