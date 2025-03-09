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

pub struct FixCommand {
    names: Vec<&'static str>,
}

pub enum FixCommandTargets {
    All,
    Tags(Vec<String>),
}

pub struct FixCommandOptions {
    pub targets: Option<FixCommandTargets>,
}

impl FixCommand {
    pub fn new() -> FixCommand {
        FixCommand {
            names: vec!["/fix"],
        }
    }
}

impl Command for FixCommand {
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
        panic!("Not implemented: /fix");
    }
}
