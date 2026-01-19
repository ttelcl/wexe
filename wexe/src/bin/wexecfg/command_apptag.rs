use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::wexe_repository::WexeEntry;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::WexeRepository;

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

fn create_apptag_file(tag: &String, entry: &WexeEntry) {
    //let target_exe = entry.get_target_exe_path().clone().unwrap();
    //let target_exe_text = target_exe.to_string_lossy();
    let stamp_option = entry.get_target_stamp();
    match stamp_option {
        None => {
            eprintln!(
                "{fg_c}{tag:>20}{fg_W} : {fg_r}Unable to access target file time stamp. Skipping{rst}."
            );
            return;
        }
        Some(stamp) => {
            let apptag_name_text = tag.to_owned() + ".apptag";
            let apptag_name = PathBuf::from(&apptag_name_text);
            eprintln!(
                "Creating or updating {fg_g}{apptag_name_text}{rst} and applying time stamp {fg_b}{stamp}{rst}."
            );
            let file = File::create(apptag_name).unwrap();
            file.set_modified(stamp.into()).unwrap();
            return;
        }
    }
}

fn create_apptag_makefile(tag: &String, entry: &WexeEntry) {
    let target_exe = entry.get_target_exe_path().clone().unwrap();
    let target_exe_text = target_exe.to_string_lossy();
    // Escape spaces in the target name. GNU make is really bad with spaces in filenames,
    // but at least it is worth a try
    let target_exe_text_escaped = target_exe_text.replace(' ', "\\ ");
    let apptag_name_text = tag.to_owned() + ".apptag";
    let mk_name_text = tag.to_owned() + ".apptag.mk";
    let mk_name = PathBuf::from(&mk_name_text);
    eprintln!("Writing {fg_g}{mk_name_text}{rst}.");
    let mut file = File::create(mk_name).unwrap();
    writeln!(&mut file, "# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.").unwrap();
    writeln!(&mut file, "# include this makefile in your main makefile").unwrap();
    writeln!(&mut file).unwrap();
    writeln!(&mut file, "{apptag_name_text} : {target_exe_text_escaped}").unwrap();
    writeln!(&mut file, "\twexecfg /apptag {tag}").unwrap();
    writeln!(&mut file).unwrap();
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
        let repo = WexeRepository::new();
        for tag in options.targets {
            let entry = repo.find_entry(&tag);
            match entry {
                Some(entry) => {
                    if !entry.target_exists() {
                        eprintln!(
                            "{fg_c}{tag:>20}{fg_W} : {fg_r}App target executable missing. Skipping{rst}."
                        );
                        continue;
                    }
                    if options.makefilemode {
                        create_apptag_makefile(&tag, entry);
                    } else {
                        create_apptag_file(&tag, entry);
                    }
                }
                None => {
                    eprintln!("{fg_c}{tag:>20}{fg_W} : {fg_r}Unknown application. Skipping{rst}.");
                    continue;
                }
            }
        }
        return Ok(ExitCode::SUCCESS);
    }
}
