
use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use same_file::is_same_file;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::{WexeRepository, target_missing_or_older};

use wexe::console_colors::*;

pub struct InstallCommand {
    names: Vec<&'static str>,
}

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
        InstallCommand {
            names: vec!["/install", "/i"],
        }
    }
}

impl Command for InstallCommand {
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
        let mut options = InstallCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Ok(ExitCode::FAILURE);
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
            return Ok(ExitCode::FAILURE);
        }

        let wexecfg_dest = repo.get_wexecfg_exe_path();
        let wexecfg_short_name = &exe.file_name().unwrap();
        if target_missing_or_older(&exe, &wexecfg_dest) {
            println!(
                "Copying {fg_b}{:}{fg_W} to {fg_g}{:}{rst}.",
                &exe.to_string_lossy(),
                wexecfg_dest.to_string_lossy()
            );
            fs::copy(&exe, wexecfg_dest)?;
        } else {
            println!(
                "{fg_g}{}{fg_W} is already up to date{rst}.",
                wexecfg_short_name.to_string_lossy()
            );
        }

        let wexe_dest = repo.get_wexe_exe_path();
        let wexe_short_name = wexe_dest.file_name().unwrap();
        if options.include_wexe {
            let wexe_source = {
                let mut wexe_source = exe.clone();
                wexe_source.set_file_name(wexe_short_name);
                wexe_source
            };
            if target_missing_or_older(&wexe_source, &wexe_dest) {
                println!(
                    "Copying {fg_b}{:}{fg_W} to {fg_g}{:}{rst}.",
                    &wexe_source.to_string_lossy(),
                    wexe_dest.to_string_lossy()
                );
                fs::copy(&wexe_source, wexe_dest)?;
            } else {
                println!(
                    "{fg_g}{}{fg_W} is already up to date{rst}.",
                    wexe_short_name.to_string_lossy()
                );
            }
        } else {
            if target_missing_or_older(&exe, &wexe_dest) {
                println!(
                    "{fg_o}{}{fg_y} not copied. Use {fg_g}-wexe{fg_y} to include it{rst}.",
                    wexe_short_name.to_string_lossy()
                );
            } // else : stay silent. It was not requested but is up to date already.
        }

        Ok(ExitCode::SUCCESS)
    }
}
