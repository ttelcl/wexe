#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use same_file::is_same_file;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::{WexeRepository, target_missing_or_older};

use wexe::console_colors::*;

pub struct DropCommand {
    names: Vec<&'static str>,
}

pub struct DropCommandOptions {
    pub targets: Vec<String>,
}

impl DropCommand {
    pub fn new() -> DropCommand {
        DropCommand {
            names: vec!["/drop", "/delete", "/rm", "/del"],
        }
    }
}

impl Command for DropCommand {
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
        panic!("Not implemented: /drop");
    }
}
