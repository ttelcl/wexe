use std::error::Error;
use std::process::ExitCode;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
//use super::wexe_repository::WexeRepository;
//use super::wexe_repository::get_file_stamp;

use wexe::config_model::is_valid_app_tag;
use wexe::console_colors::*;

pub struct ApptagCommand {
    names: Vec<&'static str>,
}

struct ApptagCommandOptions {
    pub makefilemode: bool,
    pub targets: Vec<String>,
}

impl ApptagCommandOptions {
    pub fn new() -> ApptagCommandOptions {
        ApptagCommandOptions {
            makefilemode: false,
            targets: Vec::new(),
        }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-mk" => {
                    self.makefilemode = true;
                    args.skip(1);
                }
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

impl ApptagCommand {
    pub fn new() -> ApptagCommand {
        ApptagCommand {
            names: vec!["/apptag", "/apptags", "/tag", "/tags"],
        }
    }
}

impl Command for ApptagCommand {
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
        let mut options = ApptagCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Ok(ExitCode::FAILURE);
        }
        eprintln!(
            "{fg_o}The {fg_y}/apptag{fg_o} command is not yet implemented.{rst}.",
        );
        return Ok(ExitCode::FAILURE);
    }
}
