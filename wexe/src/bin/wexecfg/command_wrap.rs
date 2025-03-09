#![allow(dead_code)]
#![allow(unused_imports)]

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf, absolute};
use std::process::ExitCode;
use std::{env, option};

use same_file::is_same_file;

use wexe::config_model::is_valid_app_tag;
use wexe::console_colors::*;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::{WexeRepository, target_missing_or_older};

pub struct WrapCommand {
    names: Vec<&'static str>,
}

pub struct WrapCommandOptions {
    pub target_path: Option<PathBuf>,
    pub tag: Option<String>,
    pub force: bool,
}

impl WrapCommandOptions {
    pub fn new() -> WrapCommandOptions {
        WrapCommandOptions {
            target_path: None,
            tag: None,
            force: false,
        }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-n" | "-name" | "-tag" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the application name).{rst}",
                        );
                        return false;
                    }
                    let tag = args.get_at(1);
                    if !is_valid_app_tag(tag) {
                        eprintln!(
                            "{fg_o}Invalid application tag as argument to {fg_g}-n{fg_o}: {fg_y}{tag}{rst}."
                        );
                        return false;
                    }
                    self.tag = Some(tag.to_string());
                    args.skip(2);
                }
                "-x" | "-exe" | "-target" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the path to the target executable).{rst}",
                        );
                        return false;
                    }
                    let target = args.get_at(1);
                    match std::path::absolute(target) {
                        Ok(target_path) => {
                            #[cfg(windows)]
                            if !target_path
                                .extension()
                                .expect("Expecting target file to have a '.exe' extension")
                                .to_str()
                                .unwrap()
                                .eq_ignore_ascii_case("exe")
                            {
                                eprintln!(
                                    "{fg_o}Target executable path {fg_y}{target}{fg_o} does not have an '.exe' extension.{rst}",
                                );
                                return false;
                            }
                            if !target_path.is_file() {
                                eprintln!(
                                    "{fg_o}Target executable path {fg_y}{target}{fg_o} does not exist or is not a file.{rst}",
                                );
                                return false;
                            }
                            println!("DEBUG: Target path: {:}", target_path.to_string_lossy());
                            self.target_path = Some(target_path);
                        }
                        Err(e) => {
                            eprintln!(
                                "{fg_o}Error resolving target executable path {fg_y}{target}{fg_o}: {fg_R}{e}{rst}."
                            );
                            return false;
                        }
                    }
                    args.skip(2);
                }
                "-F" | "-force" | "--force" => {
                    self.force = true;
                    args.skip(1);
                }
                _ => {
                    eprintln!("{fg_o}Unrecognized option: {fg_y}{:}{rst}.", arg_key);
                    return false;
                }
            }
        }
        if self.target_path.is_none() {
            eprintln!(
                "{fg_o}No target executable specified. Option {fg_y}-x{fg_o} is required.{rst}"
            );
            return false;
        }
        true
    }
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
        let mut options = WrapCommandOptions::new();
        if !options.parse_args(_args) {
            _commands.print_help_for(self.name());
            return Err(format!("Invalid arguments for command '{}'.", self.name()).into());
        }
        panic!("Not implemented: /wrap");
    }
}
