use std::env;
use std::process::Command;
use std::process::Termination;
use std::{error::Error, path::PathBuf};
// use std::os::windows::process::ExitCodeExt; // not yet stable :(

use wexe::config_model::{WexeApp, get_config_file, is_valid_app_tag, read_config_file, wexe_dbg};
use wexe::console_colors::*;

fn run_app_raw(args: Vec<String>, cfg: WexeApp) -> Result<i32, Box<dyn Error>> {
    let mut extended_args: Vec<String> = Vec::new();
    extended_args.extend(cfg.args.prepend);
    extended_args.extend(args);
    extended_args.extend(cfg.args.append);

    // Test if the target executable exists. In this usage that is an error.
    // This cannot be tested earlier, because other usages may not require the target to exist.
    let target = PathBuf::from(&cfg.target);
    if !target.exists() {
        eprintln!(
            "{bg_B}Target executable does not exist: {fg_r}{:}{rst}.",
            cfg.target
        );
        let error_text = format!("Target executable does not exist: {:}", cfg.target);
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, error_text);
        return Err(Box::new(error));
    }

    let mut cmd = Command::new(cfg.target);
    cmd.args(extended_args);
    for (k, v) in cfg.env_set.iter() {
        if v.is_empty() {
            // eprintln!("{bg_B}Removing env variable {fg_o}{:}{rst}.", k);
            cmd.env_remove(k);
        } else {
            // eprintln!("{bg_B}Setting env variable {fg_o}{:}{rst} to {fg_g}{:?}{rst}.", k, v);
            cmd.env(k, v);
        }
    }
    for (k, v) in cfg.env_pathlike.iter() {
        let originals: Vec<PathBuf> = {
            let evar = env::var(k);
            match evar {
                Ok(evar) => env::split_paths(evar.as_str()).collect(),
                Err(_) => Vec::new(),
            }
        };
        let mut new_elements: Vec<PathBuf> = Vec::new();
        new_elements.extend(v.prepend.iter().map(|s| PathBuf::from(s)));
        new_elements.extend(originals.iter().map(|s| s.clone()));
        new_elements.extend(v.append.iter().map(|s| PathBuf::from(s)));

        let new_variable: String = env::join_paths(new_elements.iter())
            .unwrap()
            .into_string()
            .unwrap();
        // eprintln!("{bg_B}Edited PATH-like variable {fg_o}{:}{rst}: {fg_g}{:?}{rst}.", k, &new_variable);
        cmd.env(k, new_variable);
    }

    if wexe_dbg() {
        println!("{bg_B}Running command: {fg_g}{:?}{rst}.", cmd);
    }

    let status = cmd.status();
    match status {
        Ok(status) => {
            if status.success() {
                if wexe_dbg() {
                    eprintln!("{bg_B}Command succeeded with exit code: {fg_g}0{rst}.");
                }
            } else {
                match status.code() {
                    Some(code) => {
                        if wexe_dbg() {
                            eprintln!("{bg_B}Command returned exit code: {fg_r}{:}{rst}.", code)
                        }
                    }
                    None => eprintln!(
                        "{bg_B}Command failed with no exit code (terminated by signal){rst}."
                    ),
                }
            }
            Ok(status.code().unwrap_or(0))
        }
        Err(e) => {
            println!("{rst}Command failed with error: {fg_r}{:?}{rst}.", e);
            Err(Box::new(e))
        }
    }
}

fn run_app(tag: String, skip1: bool) -> Result<i32, Box<dyn Error>> {
    let tag = tag.to_lowercase(); // force lower case app names
    if tag == "wexe" {
        panic!("To prevent infinite recursion, 'wexe' is rejected as app name.");
    }

    if wexe_dbg() {
        eprintln!(
            "{bg_B}Running in redirect mode (app '{fg_g}{}{rst}{bg_B}'){rst}.",
            tag.clone()
        );
    }

    let skip_count = if skip1 { 2 } else { 1 };
    let args: Vec<String> = env::args().skip(skip_count).collect();

    let cfg: WexeApp = {
        if &tag == "wexecfg" {
            // Use hard-coded configuration for wexecfg. wexe itself also gets
            // redirected here, unless its first argument is a valid app name.
            let cfg = wexe::config_model::wexecfg_config_file();
            if wexe_dbg() {
                eprintln!(
                    "{bg_B}Using hardcoded config for app {fg_o}wexecfg{rst}{bg_B}: {fg_g}{:?}{rst}.",
                    cfg
                );
            }
            cfg
        } else {
            if !is_valid_app_tag(&tag) {
                eprintln!(
                    "{bg_B}Invalid application tag '{fg_r}{:}{rst}{bg_B}'{rst}.",
                    tag.clone()
                );
                let error_text = format!("Invalid app tag '{}'.", tag.clone());
                let error = std::io::Error::new(std::io::ErrorKind::InvalidInput, error_text);
                return Err(Box::new(error));
            }
            let cfg_file_opt = get_config_file(tag.clone());
            let cfg_file = match cfg_file_opt {
                Some(cfg_file) => {
                    if wexe_dbg() {
                        eprintln!(
                            "{bg_B}Config file for app {fg_o}{:}{rst}{bg_B}: {fg_g}{:}{rst}.",
                            tag.clone(),
                            cfg_file.to_string_lossy()
                        );
                    }
                    cfg_file
                }
                None => {
                    eprintln!(
                        "{bg_B}No config file found for app '{fg_r}{:}{rst}{bg_B}'{rst}.",
                        tag.clone()
                    );
                    let error_text = format!("No configuration file for '{}'.", tag.clone());
                    let error = std::io::Error::new(std::io::ErrorKind::NotFound, error_text);
                    return Err(Box::new(error));
                }
            };

            let cfg = read_config_file(cfg_file)?;
            if wexe_dbg() {
                println!(
                    "{bg_B}Config for app {fg_o}{:}{rst}{bg_B}: {fg_g}{:?}{rst}.",
                    tag.clone(),
                    cfg
                );
            }
            cfg
        }
    };

    run_app_raw(args, cfg)
}

fn run_wexe() -> Result<i32, Box<dyn Error>> {
    let first_arg = env::args().nth(1);
    match first_arg {
        Some(tag) => {
            if !tag.starts_with("-") && !tag.starts_with("/") && !tag.starts_with("+") {
                // alternative redirect mode syntax
                return run_app(tag, true);
            }
        }
        None => (),
    };
    if wexe_dbg() {
        eprintln!("{bg_B}No app specified: redirecting to {fg_o}wexecfg{rst}.");
    }
    run_app("wexecfg".to_string(), false)
}

fn mainmain() -> Result<i32, Box<dyn Error>> {
    let exe = env::current_exe()?;
    let tag = exe.file_stem().unwrap().to_str().unwrap().to_lowercase();

    if tag == "wexe" {
        // the original application name (not renamed)
        run_wexe()
    } else {
        // the application has been renamed
        run_app(tag, false)
    }
}

fn main() {
    // Small wrapper to catch errors and print them. And support returning an exit code
    // outside the 0-255 range.
    let result = mainmain();
    match result {
        Ok(code) => {
            // eprintln!("{bg_B}{fg_g}Success with code {code}.{rst}");
            std::process::exit(code)
        }
        Err(e) => {
            // eprintln!("{bg_B}{fg_r}Failed.{rst}");
            let result2: Result<(), Box<dyn Error>> = Err(e);
            // Lets not colorize the report, avoiding the risk of not reseting the colors.
            // eprint!("{bg_B}");
            let _exitcode = result2.report();
            // eprint!("{rst}");
            std::process::exit(1);
        }
    }
}
