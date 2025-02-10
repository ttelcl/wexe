use std::path::{Path, PathBuf};
use std::env;
use std::fs::create_dir_all;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use toml;

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

// Top level configuration file model
#[derive(Debug, Serialize, Deserialize)]
pub struct WexeAppConfig {
    pub target: String,
    pub env: Option<ConfigEnv>,
    pub args: Option<ConfigArgs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigArgs {
    pub prepend: Option<Vec<String>>,
    pub append: Option<Vec<String>>,
}

// Models the "env" section of the configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnv {
    pub set: Option<HashMap<String, String>>,
    pub delete: Option<Vec<String>>,
    pub pathlike: Option<ConfigEnvPathlike>,
}

// Models the "env.pathlike" section of the configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnvPathlike {
    pub prepend: Option<HashMap<String, Vec<String>>>,
    pub append: Option<HashMap<String, Vec<String>>>,
}

// The disambiguated Wexe Application configuration model
#[derive(Debug, Serialize)]
pub struct WexeApp {
    // The target executable to run
    pub target: String,
    // Arguments to prepend to the command line
    pub args_prepend: Vec<String>,
    // Arguments to append to the command line
    pub args_append: Vec<String>,
    // Environment variables to set or override
    pub env_set: HashMap<String, String>,
    // Environment variables to delete
    pub env_delete: Vec<String>,
    // Elements to prepend to PATH-like environment variables
    pub env_pathlike_prepend: HashMap<String, Vec<String>>,
    // Elements to append to PATH-like environment variables
    pub env_pathlike_append: HashMap<String, Vec<String>>,
}

pub fn read_config_file(cfg_file: PathBuf) -> WexeApp {
    let cfg_text = std::fs::read_to_string(cfg_file)
        .expect("Could not read the config file.");
    let cfg: WexeAppConfig = toml::from_str(&cfg_text)
        .expect("Could not parse the config file.");
    let env = cfg.env.unwrap_or(ConfigEnv {
        set: None,
        delete: None,
        pathlike: None,
    });
    let env_pathlike = env.pathlike.unwrap_or(ConfigEnvPathlike {
        prepend: None,
        append: None,
    });
    let arguments = cfg.args.unwrap_or(ConfigArgs {
        prepend: None,
        append: None,
    });
    let target = Path::new(&cfg.target);
    let appdef = WexeApp {
        target: target.to_str().unwrap().to_string(),
        args_prepend: arguments.prepend.unwrap_or(Vec::new()),
        args_append: arguments.append.unwrap_or(Vec::new()),
        env_set: env.set.unwrap_or(HashMap::new()),
        env_delete: env.delete.unwrap_or(Vec::new()),
        env_pathlike_prepend: env_pathlike.prepend.unwrap_or(HashMap::new()),
        env_pathlike_append: env_pathlike.append.unwrap_or(HashMap::new()),
    };
    if !target.is_absolute() {
        panic!("Target executable path must be absolute: {:?}", appdef.target);
    }
    if !target.exists() {
        panic!("Target executable does not exist: {:?}", appdef.target);
    }
    appdef
}
