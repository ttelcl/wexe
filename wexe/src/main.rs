mod config_model;

use std::error::Error;
use std::env;
//use std::path::PathBuf;

use config_model::{get_wexe_cfg_dir, get_config_file};

fn run_app(tag: String, skip1: bool) -> Result<(), Box<dyn Error>> {
    if tag == "wexe" {
        panic!("To prevent infinite recursion, 'wexe' is rejected as app name.");
    }

    println!("Running in redirect mode (app '\x1b[92m{:?}\x1b[0m').", tag.clone());
    let cfg_file = get_config_file(tag.clone());
    match cfg_file {
        Some(cfg_file) => {
            println!("Config file for app \x1b[94m{:}\x1b[0m: \x1b[92m{:?}\x1b[0m.", tag.clone(), cfg_file);
        },
        None => {
            println!("No config file found for app '\x1b[91m{:}\x1b[0m'.", tag.clone());
            let error_text = format!("No configuration file for '{}'.", tag.clone());
            let error = std::io::Error::new(std::io::ErrorKind::NotFound, error_text);
            return Err(Box::new(error));
        }
    }

    //let wexe_cfg_dir = get_wexe_cfg_dir();
    //println!("Central Config directory: \x1b[93m{:?}\x1b[0m.", wexe_cfg_dir);
    
    let skip_count = if skip1 { 2 } else { 1 };
    println!("Args (after app name):");
    for arg in env::args().skip(skip_count) {
        println!("+ {}", arg);
    }
    Ok(())
}

fn run_wexe() -> Result<(), Box<dyn Error>> {
    let first_arg = env::args().nth(1);
    match first_arg {
        Some(tag) => {
            // println!("Tag 2: \x1b[91m{:?}\x1b[0m.", tag);
            if !tag.starts_with("-") && !tag.starts_with("/") && !tag.starts_with("+") {
                // alternative redirect mode syntax
                return run_app(tag, true);
            }
        },
        None => ()
    };
    println!("Running in non-redirect mode (wexe manager).");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let wexe_cfg_dir = get_wexe_cfg_dir();
    println!("Central Config directory: \x1b[93m{:?}\x1b[0m.", wexe_cfg_dir);

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
