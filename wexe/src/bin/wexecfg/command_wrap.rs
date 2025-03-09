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

pub struct WrapCommand {
    names: Vec<&'static str>,
}

pub struct WrapCommandOptions {
    pub target_path: Option<String>,
    pub tag: Option<String>,
    pub force: bool,
}

impl WrapCommand {
    pub fn new() -> WrapCommand {
        WrapCommand {
            names: vec!["/wrap", "/w"],
        }
    }
}

impl Command for WrapCommand {
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
        panic!("Not implemented: /wrap");
    }
}
