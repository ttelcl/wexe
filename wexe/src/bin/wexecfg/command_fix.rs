#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use same_file::is_same_file;

use wexe::config_model::is_valid_app_tag;
use wexe::console_colors::*;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::{WexeRepository, target_missing_or_older};

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

impl FixCommandOptions {
    pub fn new() -> FixCommandOptions {
        FixCommandOptions { targets: None }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-all" => {
                    if self.targets.is_some() {
                        eprintln!(
                            "{fg_o}Option {fg_y}-all{fg_o} cannot be combined with other options{rst}."
                        );
                        return false;
                    }
                    self.targets = Some(FixCommandTargets::All);
                    args.skip(1);
                }
                x => {
                    if !is_valid_app_tag(x) {
                        if is_valid_app_tag(x.to_ascii_lowercase().as_str()) {
                            // Give a better error message in case the only issue is that there
                            // are upper case characters in the tag
                            eprintln!(
                                "{fg_o}Application tags must not contain upper case characters: {fg_y}{x}{rst}."
                            );
                        } else if x.starts_with('-') {
                            eprintln!(
                                "{fg_o}Unrecognized option: {fg_y}{x}{fg_o}{rst}."
                            );
                        } else {
                            eprintln!(
                                "{fg_o}Expecting {fg_y}-all{fg_o} or a valid application tag: {fg_y}{x}{fg_o} is neither{rst}."
                            );
                        }
                        return false;
                    }
                    if x == "wexe" || x == "wexecfg" {
                        eprintln!(
                            "{fg_o}The tags {fg_y}wexe{fg_o} and {fg_y}wexecfg{fg_o} are reserved and cannot be used for applications{rst}."
                        );
                        return false;
                    }
                    match self.targets {
                        Some(FixCommandTargets::Tags(ref mut tags)) => {
                            tags.push(x.to_string());
                        }
                        Some(FixCommandTargets::All) => {
                            eprintln!(
                                "{fg_o}Option {fg_y}-all{fg_o} cannot be combined with other arguments: {fg_y}{x}{rst}."
                            );
                            return false;
                        }
                        None => {
                            self.targets = Some(FixCommandTargets::Tags(vec![x.to_string()]));
                        }
                    }
                    args.skip(1);
                }
            }
        }
        true
    }
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
        args: &mut ArgumentsBuffer,
        commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = FixCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Err(format!("Invalid arguments for command '{}'.", self.name()).into());
        }
        match options.targets {
            None => {
                eprintln!(
                    "{fg_o}No target(s) specified for command {fg_y}{:}{rst}.",
                    self.name()
                );
                commands.print_help_for(self.name());
                return Err("No target specified.".into());
            }
            Some(FixCommandTargets::All) => {
                panic!("Not implemented: /fix -all");
            }
            Some(FixCommandTargets::Tags(_tags)) => {
                panic!("Not implemented: /fix (appnames)");
            }
        }
    }
}
