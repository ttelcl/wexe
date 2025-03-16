use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

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
                tag => {
                    if !is_valid_app_tag(tag) {
                        if is_valid_app_tag(tag.to_ascii_lowercase().as_str()) {
                            // Give a better error message in case the only issue is that there
                            // are upper case characters in the tag
                            eprintln!(
                                "{fg_o}Application tags must not contain upper case characters: {fg_y}{tag}{rst}."
                            );
                        } else if tag.starts_with('-') {
                            eprintln!("{fg_o}Unrecognized option: {fg_y}{tag}{fg_o}{rst}.");
                        } else {
                            eprintln!(
                                "{fg_o}Expecting {fg_y}-all{fg_o} or a valid application tag: {fg_y}{tag}{fg_o} is neither{rst}."
                            );
                        }
                        return false;
                    }
                    if tag == "wexe" || tag == "wexecfg" {
                        eprintln!(
                            "{fg_o}The tags {fg_y}wexe{fg_o} and {fg_y}wexecfg{fg_o} are reserved and cannot be used for applications{rst}."
                        );
                        return false;
                    }
                    match self.targets {
                        Some(FixCommandTargets::Tags(ref mut tags)) => {
                            tags.push(tag.to_string());
                        }
                        Some(FixCommandTargets::All) => {
                            eprintln!(
                                "{fg_o}Option {fg_y}-all{fg_o} cannot be combined with other arguments: {fg_y}{tag}{rst}."
                            );
                            return false;
                        }
                        None => {
                            self.targets = Some(FixCommandTargets::Tags(vec![tag.to_string()]));
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

fn fix_tag(repo: &WexeRepository, tag: &str) -> Result<(), Box<dyn Error>> {
    if tag == "wexe" || tag == "wexecfg" {
        // This case should have been handled by the argument parsing already,
        // but just in case we get here, we'll print a message and return.
        eprintln!("{fg_m}{tag:>20}{fg_W} : {fg_k}Skipping reserved application tag{rst}.");
        return Ok(());
    }
    let entry = repo.find_entry(tag);
    match entry {
        Some(entry) => {
            // existing entry: ensure the stub exists and is up to date
            // (or ensure it is removed if the appdef is broken)
            if let Some(err) = entry.get_load_error() {
                let stub_path = entry.get_stub_exe_path();
                if stub_path.exists() {
                    println!("{fg_k}{tag:>20}{fg_W} : {fg_r}Removing broken stub. {fg_o}{err}{rst}.");
                    fs::remove_file(stub_path)?;
                } else {
                    println!(
                        "{fg_k}{tag:>20}{fg_W} : {fg_k}Not creating stub for broken application definition. {fg_o}{err}{rst}."
                    );
                }
                return Ok(());
            }
            if !entry.target_exists() {
                let stub_path = entry.get_stub_exe_path();
                if stub_path.exists() {
                    println!("{fg_k}{tag:>20}{fg_W} : {fg_r}Removing broken stub. {fg_o}Target executable does not exist{rst}.");
                    fs::remove_file(stub_path)?;
                } else {
                    println!(
                        "{fg_k}{tag:>20}{fg_W} : {fg_k}Not creating stub for broken application definition. {fg_o}Target executable does not exist{rst}."
                    );
                }
                return Ok(());
            }
            let wexe_path = repo.get_wexe_exe_path();
            if !wexe_path.exists() {
                eprintln!(
                    "{fg_r}WEXE executable not found: {fg_y}{:} {fg_o}Missing call to {fg_y}wexecfg /install{rst}?",
                    wexe_path.to_string_lossy()
                );
                return Err("WEXE executable not installed.".into());
            }
            let stub_path = entry.get_stub_exe_path();
            let fix_needed = target_missing_or_older(&wexe_path, &stub_path);
            if fix_needed {
                if stub_path.exists() {
                    println!("{fg_c}{tag:>20}{fg_W} : {fg_b}Updating existing stub{rst}.");
                    fs::copy(&wexe_path, &stub_path)?;
                } else {
                    println!("{fg_c}{tag:>20}{fg_W} : {fg_y}Creating missing stub{rst}.");
                    fs::copy(&wexe_path, &stub_path)?;
                }
            } else {
                println!("{fg_g}{tag:>20}{fg_W} : {fg_G}Stub is already up to date{rst}.");
            }
            Ok(())
        }
        None => {
            // Missing entry: ensure there is no dangling stub
            let stub_path = repo.get_stub_path(tag);
            if stub_path.exists() {
                println!("{fg_o}{tag:>20}{fg_W} : {fg_y}Removing orphaned stub{rst}.");
                fs::remove_file(stub_path)?;
            } else {
                println!("{fg_b}{tag:>20}{fg_W} : {fg_y}No such application or stub{rst}.");
            }
            Ok(())
        }
    }
}

fn fix_tags(tags: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let repo: WexeRepository = WexeRepository::new();
    for tag in tags {
        fix_tag(&repo, tag)?;
    }
    Ok(())
}

fn fix_all() -> Result<(), Box<dyn Error>> {
    let repo: WexeRepository = WexeRepository::new();
    let mut tags_set: BTreeSet<String> = BTreeSet::new();
    for entry in repo.get_entries() {
        tags_set.insert(entry.get_tag().to_string());
    }
    for direntry in fs::read_dir(repo.get_config_folder()).unwrap() {
        let entry = direntry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let is_exe: bool;
            match path.extension() {
                Some(ext) => {
                    if ext.to_ascii_lowercase() == std::env::consts::EXE_EXTENSION {
                        is_exe = true;
                    } else {
                        is_exe = false;
                    }
                }
                None => {
                    is_exe = std::env::consts::EXE_EXTENSION == "";
                }
            }
            if is_exe {
                let tag = path.file_stem().unwrap().to_string_lossy().to_string();
                if tag != tag.to_lowercase() {
                    eprintln!(
                        "{fg_r}Skipping executable containing upper case characters in its name: {fg_y}{}{rst} ({fg_m}manual fix required{rst}).",
                        path.to_string_lossy()
                    );
                    continue;
                }
                if is_valid_app_tag(&tag) {
                    tags_set.insert(tag.to_string());
                }
            }
        }
    }
    let tags = tags_set.into_iter().collect();
    fix_tags(&tags)?;
    Ok(())
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
            return Ok(ExitCode::FAILURE);
        }
        match options.targets {
            None => {
                eprintln!(
                    "{fg_o}No target(s) specified for command {fg_y}{:}{rst}.",
                    self.name()
                );
                commands.print_help_for(self.name());
                Err("No target specified.".into())
            }
            Some(FixCommandTargets::All) => {
                fix_all()?;
                Ok(ExitCode::SUCCESS)
            }
            Some(FixCommandTargets::Tags(tags)) => {
                fix_tags(&tags)?;
                Ok(ExitCode::SUCCESS)
            }
        }
    }
}
