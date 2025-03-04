#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::env::consts::EXE_SUFFIX;
use std::path::PathBuf;
use std::fs::read_dir;
use std::fs::DirEntry;

use wexe::config_model::read_config_file;
use wexe::config_model::{
    get_config_file, get_wexe_cfg_dir, is_valid_app_tag,
};
use wexe::console_colors::*;

pub struct WexeRepository {
    config_folder: PathBuf,
    // bin_folder: PathBuf,
    wexe_exe_path: PathBuf,
    wexecfg_exe_path: PathBuf,
    entries: BTreeMap<String, WexeEntry>,
}

pub struct WexeEntry {
    pub tag: String,
    stub_exe_path: PathBuf,
    cfg_path: PathBuf,
    target_exe_path: Option<PathBuf>, // None if configuration loading failed
    load_error: Option<String>,       // None if configuration loading succeeded
}

impl WexeEntry {
    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    pub fn get_stub_exe_path(&self) -> &PathBuf {
        &self.stub_exe_path
    }

    pub fn get_cfg_path(&self) -> &PathBuf {
        &self.cfg_path
    }

    pub fn get_target_exe_path(&self) -> &Option<PathBuf> {
        &self.target_exe_path
    }

    pub fn get_load_error(&self) -> &Option<String> {
        &self.load_error
    }
}

impl WexeRepository {
    pub fn new() -> WexeRepository {
        let config_folder = get_wexe_cfg_dir();
        // let bin_folder = get_wexe_cfg_bin_dir();
        let mut entries = BTreeMap::new();
        for direntry in read_dir(&config_folder).unwrap() {
            let entry = direntry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "toml" {
                        let tag = path.file_stem().unwrap().to_str().unwrap().to_string();
                        if is_valid_app_tag(&tag) {
                            let cfg_path = path;
                            let stub_exe_path = config_folder.join(tag.clone() + EXE_SUFFIX);
                            let wexeapp = read_config_file(cfg_path.clone());
                            let target_exe_path;
                            let load_error;
                            match wexeapp {
                                Ok(app) => {
                                    target_exe_path = Some(PathBuf::from(app.target));
                                    load_error = None;
                                }
                                Err(e) => {
                                    // eprintln!(
                                    //     "{stl_i}{fg_R}Error reading configuration file for app {fg_o}{:}{rst}: {fg_R}{:}{rst}.",
                                    //     tag.clone(),
                                    //     e
                                    // );
                                    target_exe_path = None;
                                    load_error = Some(e.to_string());
                                }
                            }
                            entries.insert(
                                tag.clone(),
                                WexeEntry {
                                    tag,
                                    stub_exe_path,
                                    cfg_path,
                                    target_exe_path,
                                    load_error,
                                },
                            );
                        }
                    }
                }
            }
        }
        WexeRepository {
            config_folder,
            // bin_folder,
            wexe_exe_path: get_wexe_cfg_dir().join("wexe".to_string() + EXE_SUFFIX),
            wexecfg_exe_path: get_wexe_cfg_dir().join("wexecfg".to_string() + EXE_SUFFIX),
            entries,
        }
    }

    /// Test if the given tag is a valid wexe application tag.
    pub fn is_valid_app_tag(&self, tag: &str) -> bool {
        is_valid_app_tag(tag)
    }

    /// Get the path to the root wexe configuration folder (where wexe configuration files
    /// and app stubs are stored)
    pub fn get_config_folder(&self) -> &PathBuf {
        &self.config_folder
    }

    // /// Get the path to the wexe binary folder (where wexe and wexecfg themselves are installed)
    // pub fn get_bin_folder(&self) -> &PathBuf {
    //     &self.bin_folder
    // }

    /// Get the file path to the installed wexe executable.
    pub fn get_wexe_exe_path(&self) -> &PathBuf {
        &self.wexe_exe_path
    }

    /// Get the file path to the installed wexecfg executable.
    pub fn get_wexecfg_exe_path(&self) -> &PathBuf {
        &self.wexecfg_exe_path
    }

    /// Enumerate all wexe entries in the repository.
    pub fn get_entries(&self) -> Vec<&WexeEntry> {
        self.entries.values().collect()
    }
}
