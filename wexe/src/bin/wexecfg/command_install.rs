#![allow(unused_imports)]

use std::error::Error;
use std::process::ExitCode;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::WexeRepository;
use super::wexe_repository::get_file_stamp;

use wexe::console_colors::*;

pub struct InstallCommand {}

pub struct InstallCommandOptions {
    pub include_wexe: bool,
}

impl InstallCommandOptions {
    pub fn new() -> InstallCommandOptions {
        InstallCommandOptions {
            include_wexe: false,
        }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-wexe" | "-w" => {
                    self.include_wexe = true;
                    args.skip(1);
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

impl InstallCommand {
    pub fn new() -> InstallCommand {
        InstallCommand {}
    }
}

impl Command for InstallCommand {
    fn name(&self) -> &str {
        "/install"
    }

    fn name_and_aliases(&self) -> &[&str] {
        &["/install", "/i"]
    }

    fn execute(
        &self,
        args: &mut ArgumentsBuffer,
        commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = InstallCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Err(format!("Invalid arguments for command '{}'.", self.name()).into());
        }

        //let repo = WexeRepository::new();
        panic!("{fg_o}Not implemented yet: /install{rst}.");
        //Ok(ExitCode::SUCCESS)
    }
}
