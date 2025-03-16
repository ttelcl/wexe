#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use wexe::config_model::is_valid_app_tag;
use wexe::console_colors::*;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::{WexeRepository, target_missing_or_older};

pub struct DropCommand {
    names: Vec<&'static str>,
}

pub struct DropCommandOptions {
    pub targets: Vec<String>,
}

impl DropCommandOptions {
    pub fn new() -> DropCommandOptions {
        DropCommandOptions {
            targets: Vec::new(),
        }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                tag => {
                    if !is_valid_app_tag(&tag) {
                        eprintln!(
                            "{fg_o}Expecting valid application tags as arguments: {fg_y}{tag}{fg_o} is not a valid tag{rst}."
                        );
                        return false;
                    }
                    self.targets.push(tag.to_string());
                    args.skip(1);
                }
            }
        }
        if self.targets.is_empty() {
            eprintln!("{fg_o}Expecting at least one application tag as argument{rst}.");
            return false;
        }
        true
    }
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
        args: &mut ArgumentsBuffer,
        commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = DropCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Ok(ExitCode::FAILURE);
        }
        let repo = WexeRepository::new();
        for tag in options.targets {
            let cfg_path = repo.get_config_path(&tag);
            let stub_path = repo.get_stub_path(&tag);
            let cfg_exists = cfg_path.exists();
            let stub_exists = stub_path.exists();
            if !cfg_exists && !stub_exists {
                eprintln!(
                    "{fg_k}{tag:>20}{fg_W} : {fg_o}Unknown app {rst}(No configuration nor stub exists){rst}."
                );
                continue;
            }
            if cfg_exists {
                let cfg_bak = cfg_path.with_extension("toml.bak");
                fs::rename(&cfg_path, &cfg_bak)?;
                eprintln!(
                    "{fg_c}{tag:>20}{fg_W} : {fg_w}Configuration file removed{rst}."
                );
            }
            if stub_exists {
                fs::remove_file(&stub_path)?;
                eprintln!(
                    "{fg_c}{tag:>20}{fg_W} : {fg_w}Stub deleted{rst}."
                );
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}
