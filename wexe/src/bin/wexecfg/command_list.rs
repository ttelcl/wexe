use core::panic;
use std::{error::Error, process::ExitCode};

use crate::{
    args_buffer::ArgumentsBuffer,
    commands::{Command, CommandCollection},
};

use wexe::console_colors::*;

pub struct ListCommand {
    names: Vec<&'static str>,
}

struct ListCommandOptions {
    pub filter: Option<String>,
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            names: vec!["/list", "/l"],
        }
    }
}

impl ListCommandOptions {
    pub fn new() -> ListCommandOptions {
        ListCommandOptions { filter: None }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-m" => {
                    if args.remaining() < 2 {
                        eprintln!("{fg_o}Option {fg_y}-m{fg_o} requires an argument.{rst}",);
                        return false;
                    }
                    self.filter = Some(args.get_at(1).to_string());
                    args.skip(2);
                }
                _ => {
                    eprintln!("{fg_o}Unrecognized option: {fg_y}{:}{rst}.", arg_key);
                    return false;
                }
            }
        }
        true
    }
}

impl Command for ListCommand {
    fn name(&self) -> &str {
        self.names[0]
    }

    fn name_and_aliases(&self) -> &[&str] {
        self.names.as_ref()
    }

    fn execute(
        &self,
        _args: &mut ArgumentsBuffer,
        _commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = ListCommandOptions::new();
        if !options.parse_args(_args) {
            return Err(format!("Invalid arguments for command '{}'.", self.name()).into());
        }
        panic!("{fg_r}Not yet implemented{rst}: ListCommand.execute.");
    }

    fn print_help(&self) -> () {
        println!(
            r#"{fg_o}/list{rst} [{fg_g}-m {fg_c}{stl_i}filter{rst}]\n 
    List the registered applications."#
        );
    }
}
