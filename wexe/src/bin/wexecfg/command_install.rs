#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use same_file::is_same_file;

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
        let exe = env::current_exe().expect("Could not get the current executable path.");
        let current_exe_folder = exe
            .parent()
            .expect("Could not get the executable's parent folder.");
        let repo = WexeRepository::new();
        let cfg_folder = repo.get_config_folder();

        if is_same_file(&current_exe_folder, &cfg_folder).unwrap() {
            println!(
                "{fg_r}Error!{fg_o} The current exe folder is the same as the config folder{rst}. \
                \n{fg_y}Run {fg_o}wexecfg /install{fg_y} from the directory where it was built{rst}."
            );
            return Err("Current exe folder is the same as the config folder.".into());
        }

        let wexecfg_dest = repo.get_wexecfg_exe_path();
        println!(
            "Copying {fg_b}{:}{fg_W} to {fg_g}{:}{rst}.",
            &exe.to_string_lossy(),
            wexecfg_dest.to_string_lossy()
        );
        fs::copy(&exe, wexecfg_dest)?;

        let wexe_dest = repo.get_wexe_exe_path();
        if options.include_wexe {
            println!(
                "Copying {fg_b}{:}{fg_W} to {fg_g}{:}{rst}.",
                &exe.to_string_lossy(),
                wexe_dest.to_string_lossy()
            );
            fs::copy(exe, wexe_dest)?;
        } else {
            println!(
                "{fg_o}{}{fg_y} not copied. Use {fg_g}-wexe{fg_y} to include it{rst}.",
                &exe.to_string_lossy()
            );
        }

        Ok(ExitCode::SUCCESS)
    }
}
