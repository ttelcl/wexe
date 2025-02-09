use std::error::Error;
use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;

/*
Notes and reminders on APIs that may be of use:
- env::current_exe() -> Result<PathBuf>
- dirs crate (6.0.0): dirs::config_local_dir() -> Option<PathBuf>
 */

fn get_wexe_cfg_dir() -> PathBuf {
    let mut user_cfg_dir =
        dirs::config_local_dir()
        .expect("This system has no local config directory.");
    user_cfg_dir.push(".wexe");
    create_dir_all(user_cfg_dir.as_path())
        .expect("Could not create the .wexe config directory.");
    user_cfg_dir
}

fn run_app(tag: String, skip1: bool) -> Result<(), Box<dyn Error>> {
    if tag == "wexe" {
        panic!("To prevent infinite recursion, 'wexe' is rejected as app name.");
    }

    println!("Running in redirect mode (app '\x1b[95m{:?}\x1b[0m').", tag);
    let exe = env::current_exe()?;
    println!("Current executable: \x1b[92m{:?}\x1b[0m.", exe);
    let toml_name = exe.with_extension("toml");
    println!("Local Config file: \x1b[91m{:?}\x1b[0m.", toml_name);
    // let cfg_naked = toml_name.file_name().unwrap();
    // let cfg_file = wexe_cfg_dir.join(cfg_naked);
    // println!("App config file is: \x1b[94m{:?}\x1b[0m.", cfg_file);

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
            println!("Tag 2: \x1b[91m{:?}\x1b[0m.", tag);
            if !tag.starts_with("-") {
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
    println!("Current executable: \x1b[92m{:?}\x1b[0m.", exe);
    let tag = exe.file_stem().unwrap().to_str().unwrap().to_lowercase();
    println!("Tag: \x1b[94m{:?}\x1b[0m.", tag);

    if tag == "wexe" {
        // the original application name (not renamed)
        run_wexe()
    } else {
        // the application has been renamed
        run_app(tag, false)
    }
}
