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

fn main() -> Result<(), Box<dyn Error>> {
    let wexe_cfg_dir = get_wexe_cfg_dir();
    println!("Central Config directory: {:?}", wexe_cfg_dir);
    let exe = env::current_exe()?;
    println!("Current executable: {:?}", exe);
    let toml_name = exe.with_extension("toml");
    println!("Local Config file: {:?}", toml_name);
    let cfg_naked = toml_name.file_name().unwrap();
    let cfg_file = wexe_cfg_dir.join(cfg_naked);
    println!("App config file is: {:?}", cfg_file);
    
    println!("Args:");
    for arg in env::args() {
        println!("+ {}", arg);
    }
    Ok(())
}
