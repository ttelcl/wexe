use std::path::PathBuf;
use std::env;
use std::fs::create_dir_all;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
// use toml;

pub fn get_wexe_cfg_dir() -> PathBuf {
    let mut user_cfg_dir =
        dirs::config_local_dir()
        .expect("This system has no local config directory.");
    user_cfg_dir.push(".wexe");
    create_dir_all(user_cfg_dir.as_path())
        .expect("Could not create the .wexe config directory.");
    user_cfg_dir
}

pub fn get_config_file(tag: String) -> Option<PathBuf> {
    let exe = env::current_exe().unwrap();
    let folder = exe.parent().unwrap();
    let cfg_file = folder.join(tag.clone() + ".toml");
    if cfg_file.exists() {
        return Some(cfg_file);
    }
    let wexe_cfg_dir = get_wexe_cfg_dir();
    let cfg_file = wexe_cfg_dir.join(tag.clone() + ".toml");
    if cfg_file.exists() {
        return Some(cfg_file);
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WexeAppConfig {
    pub target: String,
    pub env: ConfigEnv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnv {
    pub set: HashMap<String, String>,
    pub delete: Vec<String>,
    pub pathlike: ConfigEnvPathlike,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnvPathlike {
    pub prepend: HashMap<String, String>,
    pub append: HashMap<String, String>,
}
