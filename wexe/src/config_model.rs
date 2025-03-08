use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use toml;

use crate::console_colors::*;

pub struct WexeConfigFolders {
    pub wexe_cfg_dir: PathBuf,
    // pub wexe_bin_dir: PathBuf,
}

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
                    eprintln!(
                        "{bg_B}{stl_u}{fg_r}WEXE_DEBUG is OFF. This message only shows in debug builds, and only if WEXE_DEBUG is not set at all{rst}."
                    );
                    false
                }
                #[cfg(not(debug_assertions))]
                {
                    false
                }
            }
        }
    };
    static ref WEXE_CFG_FOLDERS: WexeConfigFolders = {
        let mut user_cfg_dir =
            dirs::config_local_dir().expect("This system has no local config directory.");
        user_cfg_dir.push(".wexe");
        fs::create_dir_all(user_cfg_dir.as_path())
            .expect("Could not create the .wexe config directory.");
        // let mut user_cfg_bin_dir = user_cfg_dir.clone();
        //user_cfg_bin_dir.push("bin");
        //fs::create_dir_all(user_cfg_bin_dir.as_path())
        //    .expect("Could not create the .wexe/bin config directory.");
        WexeConfigFolders {
            wexe_cfg_dir: user_cfg_dir,
            // wexe_bin_dir: user_cfg_bin_dir,
        }
    };
    static ref APP_TAG_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9]*([-_][a-z0-9]+)*$").unwrap();
}

/// Returns true if the wexe debug flag is set.
/// This is set if the environment variable WEXE_DEBUG is set to "1",
/// or if the program is built in debug mode and WEXE_DEBUG is not set.
/// # Returns
/// True if the wexe debug flag is set.
pub fn wexe_dbg() -> bool {
    *WEXE_DEBUG
}

/// Returns true if the given tag is a valid wexe application tag.
pub fn is_valid_app_tag(tag: &str) -> bool {
    APP_TAG_REGEX.is_match(tag)
}

/// Get the path to the wexe configuration directory in the user's local config directory.
/// Creates the directory if it does not exist yet.
/// # Returns
/// The path to the .wexe configuration directory.
pub fn get_wexe_cfg_dir() -> PathBuf {
    // let mut user_cfg_dir =
    //     dirs::config_local_dir().expect("This system has no local config directory.");
    // user_cfg_dir.push(".wexe");
    // fs::create_dir_all(user_cfg_dir.as_path())
    //     .expect("Could not create the .wexe config directory.");
    // let mut user_cfg_bin_dir = user_cfg_dir.clone();
    // user_cfg_bin_dir.push("bin");
    // fs::create_dir_all(user_cfg_bin_dir.as_path())
    //     .expect("Could not create the .wexe/bin config directory.");
    // user_cfg_dir
    WEXE_CFG_FOLDERS.wexe_cfg_dir.clone()
}

// /// Get the path to the wexe binary folder (creating it if it does not exist).
// /// This is the folder where the original wexe.exe and wexecfg.exe are installed.
// pub fn get_wexe_cfg_bin_dir() -> PathBuf {
//     WEXE_CFG_FOLDERS.wexe_bin_dir.clone()
// }

/// Get the path to a configuration file for a given tag, or None if no such file exists.
/// # Arguments
/// * `tag` - The tag to use to find the configuration file.
/// # Returns
/// The path to the configuration file, or None if no such file exists.
pub fn get_config_file(tag: String) -> Option<PathBuf> {
    if !is_valid_app_tag(&tag) {
        panic!("Invalid application tag: {}", tag);
    }
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

/// Top level configuration file model. This models the actual content of the
/// configuration file, including some optional parts.
/// See the [WexeApp] struct for the disambiguated model.
#[derive(Debug, Serialize, Deserialize)]
pub struct WexeAppConfig {
    /// The target executable to run
    // ((currently required; may be optional in the future once we get 'include' implemented))
    pub target: String,
    /// Environment variable related sections
    pub env: Option<ConfigEnv>,
    /// Arguments to prepend and append to the command line
    pub args: Option<ConfigArgs>,
}

/// Optional lists of elements to prepend or append to some existing string list
/// (be they the arguments list or a PATH-like environment variable).
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigArgs {
    pub prepend: Option<Vec<String>>,
    pub append: Option<Vec<String>>,
}

/// Models the "env" section of the configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEnv {
    /// Environment variables to set, override, or delete.
    /// An environment variable is deleted if its value is an empty string.
    pub set: Option<HashMap<String, String>>,
    /// Sections for mofifying PATH-like environment variables.
    pub pathlike: Option<HashMap<String, ConfigArgs>>,
}

/// Lists of elements to prepend or append to some existing string list
/// (be they the arguments list or a PATH-like environment variable).
#[derive(Debug, Serialize)]
pub struct ListOps {
    // elements to prepend to the list
    pub prepend: Vec<String>,
    // elements to append to the list
    pub append: Vec<String>,
}

/// The disambiguated Wexe Application configuration model, derived from the
/// [WexeAppConfig] model described by the TOML configuration file.
#[derive(Debug, Serialize)]
pub struct WexeApp {
    /// The target executable to run
    pub target: String,
    /// Arguments to prepend and append to the command line
    pub args: ListOps,
    /// Environment variables to set, override, or delete
    pub env_set: HashMap<String, String>,
    /// Prepending or appending elements to environment variables that are PATH-like
    /// Prepending or appending uses an operating-system-specific separator.
    pub env_pathlike: HashMap<String, ListOps>,
}

/// Read a TOML wexe configuration file and return a disambiguated [WexeApp] model for it.
/// # Arguments
/// * `cfg_file` - The path to the configuration file to read.
/// # Returns
/// A [WexeApp] model derived from the configuration file.
pub fn read_config_file(cfg_file: PathBuf) -> Result<WexeApp, Box<dyn Error>> {
    let cfg_text = std::fs::read_to_string(cfg_file)?;
    let cfg: WexeAppConfig = toml::from_str(&cfg_text)?;
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
        target: target.to_string_lossy().to_string(),
        args: arg_ops,
        env_set: env.set.unwrap_or(HashMap::new()),
        env_pathlike: env_pathlike_ops,
    };
    if !target.is_absolute() {
        let error_msg = format!(
            "Target executable path must be absolute: {:}",
            appdef.target
        );
        return Err(error_msg.into());
    }
    // if !target.exists() {
    //     let error_msg = format!("Target executable does not exist: {:}", appdef.target);
    //     return Err(error_msg.into());
    // }
    Ok(appdef)
}

/// Get the configuration file for the wexecfg application. This configuration is
/// hardcoded here, derived from the wexe executable location.
pub fn wexecfg_config_file() -> WexeApp {
    let exe = env::current_exe().expect("Could not get the current executable path.");
    let folder = exe
        .parent()
        .expect("Could not get the executable's parent folder.");
    let extension = exe.extension();
    let wexecfg_file_name = match extension {
        Some(ext) => format!("wexecfg.{}", ext.to_str().unwrap()),
        None => "wexecfg".to_string(),
    };
    let wexecfg_path = folder.join(wexecfg_file_name);
    if !wexecfg_path.exists() {
        panic!(
            "Could not find the wexecfg executable file: {:?}",
            wexecfg_path
        );
    }
    WexeApp {
        target: wexecfg_path.to_str().unwrap().to_string(),
        args: ListOps {
            prepend: Vec::new(),
            append: Vec::new(),
        },
        env_set: HashMap::new(),
        env_pathlike: HashMap::new(),
    }
}
