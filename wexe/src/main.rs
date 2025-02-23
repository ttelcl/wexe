mod config_model;

use std::env;
use std::process::Command;
use std::{error::Error, path::PathBuf};

use config_model::{get_config_file, read_config_file, wexe_dbg};

fn run_app(tag: String, skip1: bool) -> Result<i32, Box<dyn Error>> {
    if tag == "wexe" {
        panic!("To prevent infinite recursion, 'wexe' is rejected as app name.");
    }

    if wexe_dbg() {
        println!(
            "\x1B[44mRunning in redirect mode (app '\x1b[92m{}\x1b[0m').",
            tag.clone()
        );
    }
    let cfg_file_opt = get_config_file(tag.clone());
    let cfg_file = match cfg_file_opt {
        Some(cfg_file) => {
            if wexe_dbg() {
                println!(
                    "\x1B[44mConfig file for app \x1b[94m{:}\x1b[0m\x1B[44m: \x1b[92m{:?}\x1b[0m.",
                    tag.clone(),
                    cfg_file
                );
            }
            cfg_file
        }
        None => {
            println!(
                "No config file found for app '\x1b[91m{:}\x1b[0m'.",
                tag.clone()
            );
            let error_text = format!("No configuration file for '{}'.", tag.clone());
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, error_text);
            return Err(Box::new(error));
        }
    };

    let skip_count = if skip1 { 2 } else { 1 };
    let args: Vec<String> = env::args().skip(skip_count).collect();

    let cfg = read_config_file(cfg_file);
    if wexe_dbg() {
        println!(
            "\x1B[44mConfig for app \x1b[94m{:}\x1b[0m\x1B[44m: \x1b[92m{:?}\x1b[0m.",
            tag.clone(),
            cfg
        );
    }

    let mut extended_args: Vec<String> = Vec::new();
    extended_args.extend(cfg.args.prepend);
    extended_args.extend(args);
    extended_args.extend(cfg.args.append);
    // println!("Expanded args:");
    // for arg in extended_args.iter() {
    //     println!("+ {}", arg);
    // }

    let mut cmd = Command::new(cfg.target);
    cmd.args(extended_args);

    for delenv in cfg.env_delete.iter() {
        cmd.env_remove(delenv);
    }
    for (k, v) in cfg.env_set.iter() {
        cmd.env(k, v);
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
        // println!("Edited PATH-like variable \x1b[94m{:}\x1b[0m: \x1b[92m{:?}\x1b[0m.", k, &new_variable);
        cmd.env(k, new_variable);
    }

    if wexe_dbg() {
        println!("\x1B[44mRunning command: \x1b[92m{:?}\x1b[0m.", cmd);
    }

    let status = cmd.status();
    match status {
        Ok(status) => {
            if status.success() {
                if wexe_dbg() {
                    println!("\x1B[44mCommand succeeded with exit code: \x1b[92m0\x1b[0m.");
                }
            } else {
                match status.code() {
                    Some(code) => {
                        if wexe_dbg() {
                            println!(
                                "\x1B[44mCommand returned exit code: \x1b[91m{:}\x1b[0m.",
                                code
                            )
                        }
                    }
                    None => println!(
                        "\x1B[44mCommand failed with no exit code (terminated by signal)\x1b[0m."
                    ),
                }
            }
            Ok(status.code().unwrap_or(0))  
        }
        Err(e) => {
            println!("\x1b[0mCommand failed with error: \x1b[91m{:?}\x1b[0m.", e);
            Err(Box::new(e))
        }
    }
}

fn run_wexe() -> Result<i32, Box<dyn Error>> {
    let first_arg = env::args().nth(1);
    match first_arg {
        Some(tag) => {
            // println!("Tag 2: \x1b[91m{:?}\x1b[0m.", tag);
            if !tag.starts_with("-") && !tag.starts_with("/") && !tag.starts_with("+") {
                // alternative redirect mode syntax
                return run_app(tag, true);
            }
        }
        None => (),
    };
    println!("\x1B[44mRunning in non-redirect mode (wexe manager)\x1b[0m. \x1B[41mNYI\x1b[0m!");
    Ok(0)
}

fn main0() -> Result<i32, Box<dyn Error>> {
    // let wexe_cfg_dir = get_wexe_cfg_dir();
    // println!("Central Config directory: \x1b[93m{:?}\x1b[0m.", wexe_cfg_dir);

    let exe = env::current_exe()?;
    // println!("Current executable: \x1b[92m{:?}\x1b[0m.", exe);
    let tag = exe.file_stem().unwrap().to_str().unwrap().to_lowercase();
    // println!("Tag: \x1b[94m{:?}\x1b[0m.", tag);

    if tag == "wexe" {
        // the original application name (not renamed)
        run_wexe()
    } else {
        // the application has been renamed
        run_app(tag, false)
    }
}

fn main() {
    let result = main0();
    match result {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            println!("\x1b[91mError: {:?}\x1b[0m.", e);
            std::process::exit(1);
        }
    }
}
