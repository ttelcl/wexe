use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use super::console_colors::*;
use toml;

lazy_static! {
    static ref WEXE_DEBUG: bool = {
        match env::var("WEXE_DEBUG") {
            Ok(val) => {
                if val == "1" {
                    eprintln!("{bg_B}{stl_u}{fg_r}WEXE_DEBUG is ON (1){rst}.");
                    true
                } else {
                    eprintln!("{bg_B}{stl_u}WEXE_DEBUG is Off (not 1){rst}.");
                    false
                }
            }
            Err(_) => {
                #[cfg(debug_assertions)]
                {
                    eprintln!("{bg_B}{stl_u}{fg_r}WEXE_DEBUG is ON (WEXE_DEBUG not set, but debug build){rst}.");
                    true
                }
                #[cfg(not(debug_assertions))]
                {
                    false
                }
            }
        }
    };
}

pub fn wexe_dbg() -> bool {
    *WEXE_DEBUG
}

pub fn get_wexe_cfg_dir() -> PathBuf {
    let mut user_cfg_dir =
        dirs::config_local_dir().expect("This system has no local config directory.");
    user_cfg_dir.push(".wexe");
    create_dir_all(user_cfg_dir.as_path()).expect("Could not create the .wexe config directory.");
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
    pub pathlike: Option<HashMap<String, ConfigArgs>>,
}

#[derive(Debug, Serialize)]
pub struct ListOps {
    // elements to prepend to the list
    pub prepend: Vec<String>,
    // elements to append to the list
    pub append: Vec<String>,
}

// The disambiguated Wexe Application configuration model
#[derive(Debug, Serialize)]
pub struct WexeApp {
    // The target executable to run
    pub target: String,
    // Arguments to prepend and append to the command line
    pub args: ListOps,
    // Environment variables to set, override, or delete
    pub env_set: HashMap<String, String>,
    // Prepending or appending elements to environment variables that are PATH-like
    pub env_pathlike: HashMap<String, ListOps>,
}

pub fn read_config_file(cfg_file: PathBuf) -> WexeApp {
    let cfg_text = std::fs::read_to_string(cfg_file).expect("Could not read the config file.");
    let cfg: WexeAppConfig = toml::from_str(&cfg_text).expect("Could not parse the config file.");
    let env = cfg.env.unwrap_or(ConfigEnv {
        set: None,
        pathlike: None,
    });
    let env_pathlike = env.pathlike.unwrap_or(HashMap::new());
    let arguments = cfg.args.unwrap_or(ConfigArgs {
        prepend: None,
        append: None,
    });
    let target = Path::new(&cfg.target);
    let arg_ops = ListOps {
        prepend: arguments.prepend.unwrap_or(Vec::new()),
        append: arguments.append.unwrap_or(Vec::new()),
    };
    let env_pathlike_ops = env_pathlike
        .iter()
        .map(|(k, v)| {
            let ops = ListOps {
                prepend: v.prepend.clone().unwrap_or(Vec::new()),
                append: v.append.clone().unwrap_or(Vec::new()),
            };
            (k.clone(), ops)
        })
        .collect::<HashMap<String, ListOps>>();
    let appdef = WexeApp {
        target: target.to_str().unwrap().to_string(),
        args: arg_ops,
        env_set: env.set.unwrap_or(HashMap::new()),
        env_pathlike: env_pathlike_ops,
    };
    if !target.is_absolute() {
        panic!(
            "Target executable path must be absolute: {:?}",
            appdef.target
        );
    }
    if !target.exists() {
        panic!("Target executable does not exist: {:?}", appdef.target);
    }
    appdef
}
