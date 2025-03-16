use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use toml_edit::DocumentMut;

use wexe::config_model::is_valid_app_tag;
use wexe::console_colors::*;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::WexeRepository;

pub struct WrapCommand {
    names: Vec<&'static str>,
}

pub struct WrapCommandOptions {
    target_path: Option<PathBuf>,
    tag: Option<String>,
    pub force: bool,
    pre_args: Vec<String>,
    pre_path: Vec<String>,
}

impl WrapCommandOptions {
    pub fn new() -> WrapCommandOptions {
        WrapCommandOptions {
            target_path: None,
            tag: None,
            force: false,
            pre_args: Vec::new(),
            pre_path: Vec::new(),
        }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-n" | "-name" | "-tag" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the application name).{rst}",
                        );
                        return false;
                    }
                    let tag = args.get_at(1);
                    if !is_valid_app_tag(tag) {
                        eprintln!(
                            "{fg_o}Invalid application tag as argument to {fg_g}-n{fg_o}: {fg_y}{tag}{rst}."
                        );
                        return false;
                    }
                    self.tag = Some(tag.to_string());
                    args.skip(2);
                }
                "-x" | "-exe" | "-target" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the path to the target executable).{rst}",
                        );
                        return false;
                    }
                    let target = args.get_at(1);
                    match std::path::absolute(target) {
                        Ok(target_path) => {
                            #[cfg(windows)]
                            if !target_path
                                .extension()
                                .expect("Expecting target file to have a '.exe' extension")
                                .to_str()
                                .unwrap()
                                .eq_ignore_ascii_case("exe")
                            {
                                eprintln!(
                                    "{fg_o}Target executable path {fg_y}{target}{fg_o} does not have an '.exe' extension.{rst}",
                                );
                                return false;
                            }
                            if !target_path.is_file() {
                                eprintln!(
                                    "{fg_o}Target executable path {fg_y}{target}{fg_o} does not exist or is not a file.{rst}",
                                );
                                return false;
                            }
                            println!("DEBUG: Target path: {:}", target_path.to_string_lossy());
                            self.target_path = Some(target_path);
                        }
                        Err(e) => {
                            eprintln!(
                                "{fg_o}Error resolving target executable path {fg_y}{target}{fg_o}: {fg_R}{e}{rst}."
                            );
                            return false;
                        }
                    }
                    args.skip(2);
                }
                "-a" | "-arg" | "-preargs" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the argument to prepend).{rst}",
                        );
                        return false;
                    }
                    let arg = args.get_at(1);
                    self.pre_args.push(arg.to_string());
                    args.skip(2);
                }
                "-p" | "-path" | "-prepath" => {
                    if args.remaining() < 2 {
                        eprintln!(
                            "{fg_o}Option {fg_y}{arg_key}{fg_o} requires an argument {fg_W}(the path to prepend to PATH){rst}.",
                        );
                        return false;
                    }
                    let path = args.get_at(1);
                    match std::path::absolute(path) {
                        Ok(path) => {
                            let path_txt = path.to_string_lossy();
                            if !path.is_dir() {
                                if !path.exists() {
                                    eprintln!(
                                        "{fg_r}{arg_key}{rst} {fg_y}{path_txt}{fg_o}: Path does not exist{rst}.",
                                    );
                                } else {
                                    eprintln!(
                                        "{fg_r}{arg_key}{rst} {fg_y}{path_txt}{fg_o}: Path is not a directory{rst}.",
                                    );
                                }
                                return false;
                            }
                            self.pre_path.push(path_txt.to_string());
                        }
                        Err(e) => {
                            eprintln!(
                                "{fg_o}Error resolving absolute path to prepend to PATH {fg_y}{path}{fg_o}: {fg_R}{e}{rst}."
                            );
                            return false;
                        }
                    }
                    args.skip(2);
                }
                "-F" | "-force" | "--force" => {
                    self.force = true;
                    args.skip(1);
                }
                _ => {
                    eprintln!("{fg_o}Unrecognized option: {fg_y}{:}{rst}.", arg_key);
                    return false;
                }
            }
        }
        if self.target_path.is_none() {
            eprintln!(
                "{fg_o}No target executable specified. Option {fg_y}-x{fg_o} is required.{rst}"
            );
            return false;
        }
        true
    }

    /// Get a reference to the target path. Panics if the target path is not set.
    pub fn get_target_path(&self) -> &PathBuf {
        self.target_path.as_ref().expect("Target path not set.")
    }

    /// Get the tag/name for the application, either as set with a 'n' option,
    /// or derived from the target path.
    pub fn get_tag(&self) -> Result<String, Box<dyn Error>> {
        match &self.tag {
            Some(tag) => {
                if tag == "wexe" || tag == "wexecfg" {
                    println!(
                        "{fg_o}The tags {fg_y}wexe{fg_o} and {fg_y}wexecfg{fg_o} are reserved and cannot be used for applications{rst}."
                    );
                    Err("Invalid application tag specified.".into())
                } else {
                    Ok(tag.clone())
                }
            }
            None => {
                let target_path = self.get_target_path();
                let tag = target_path
                    .file_stem()
                    .expect("Target path has no file name.")
                    .to_str()
                    .expect("Target path file name is not valid.")
                    .to_lowercase(); // Tags must be lowercase!
                if !is_valid_app_tag(tag.as_str()) {
                    println!(
                        "{fg_o}The application tag derived from target path {fg_y}{:}{fg_o} is not valid{rst}. \
                        Please pass a valid tag with {fg_g}-n{rst}.",
                        target_path.to_string_lossy()
                    );
                    Err("Invalid application tag derived from target path.".into())
                } else if tag == "wexe" || tag == "wexecfg" {
                    println!(
                        "{fg_o}The tags {fg_y}wexe{fg_o} and {fg_y}wexecfg{fg_o} are reserved and cannot be used for applications{rst}."
                    );
                    Err("Invalid application tag derived from target path.".into())
                } else {
                    Ok(tag)
                }
            }
        }
    }
}

impl WrapCommand {
    pub fn new() -> WrapCommand {
        WrapCommand {
            names: vec!["/wrap", "/w"],
        }
    }
}

impl Command for WrapCommand {
    fn name(&self) -> &str {
        self.names[0]
    }

    fn name_and_aliases(&self) -> &[&str] {
        self.names.as_ref()
    }

    fn execute(
        &self,
        args: &mut ArgumentsBuffer,
        commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = WrapCommandOptions::new();
        if !options.parse_args(args) {
            commands.print_help_for(self.name());
            return Ok(ExitCode::FAILURE);
        }
        let target = options.get_target_path();
        let target_name = target.to_string_lossy();
        let tag = options.get_tag()?;
        println!(
            "Building application configuration '{fg_g}{tag}{rst}' targetting '{fg_c}{target_name}{rst}'."
        );

        // First build the configuration file, handle an existing file only after that.
        // let mut doc = DocumentMut::new();
        // doc["target"] = toml_edit::value(target_name.as_ref());

        let mut doc = r#"
# Additional arguments to prepend or append to the given command line.
[args]
prepend = [ ]
append = [ ]

# Plain environment variables to set or delete
[env.set]
# FOO = 'bar'
# BAR = ''

# You can prepend or append items to PATH-like environment variables
# For example: prepend or append to the PATH environment variable itself
[env.pathlike.PATH]
prepend = [ ]
append = [ ]
"#
        .parse::<DocumentMut>()
        .expect("invalid toml");
        doc["target"] = toml_edit::value(target_name.as_ref());

        let arg_prepend = doc["args"]["prepend"]
            .as_array_mut()
            .expect("args.prepend is not an array");
        for arg in options.pre_args.iter() {
            let mut v: toml_edit::Value = arg.into();
            v.decor_mut().set_prefix("\n  ");
            arg_prepend.push_formatted(v);
        }
        let path_prepend = doc["env"]["pathlike"]["PATH"]["prepend"]
            .as_array_mut()
            .expect("env.pathlike.PATH.prepend is not an array");
        for path in options.pre_path.iter() {
            let mut v: toml_edit::Value = path.into();
            v.decor_mut().set_prefix("\n  ");
            path_prepend.push_formatted(v);
        }

        let document_text = doc.to_string();
        //println!("DEBUG: Document: \n{fg_b}{document_text}{rst}");

        let repo = WexeRepository::new();
        let cfg_folder = repo.get_config_folder();
        let final_file = cfg_folder.join(tag.as_str()).with_extension("toml");
        let tmp_file = final_file.with_extension("toml.tmp");
        let bak_file = final_file.with_extension("toml.bak");
        // println!(
        //     "Saving intermediate config file: {fg_b}{:}{rst}",
        //     tmp_file.to_string_lossy()
        // );
        fs::write(&tmp_file, document_text)?;

        // Only now check if an entry for the application already exists, and use the
        // configuration accordingly.
        let existing_entry = repo.find_entry(tag.as_str());
        match existing_entry {
            Some(_entry) => {
                if options.force {
                    println!(
                        "{fg_y}An entry for application '{fg_g}{tag}{fg_y}' already exists{rst}.\
                        \n{fg_o}Backing up and overwriting {fg_y}{}{rst}.",
                        final_file.to_string_lossy()
                    );
                    fs::rename(&final_file, &bak_file)?;
                    fs::rename(&tmp_file, &final_file)?;
                } else {
                    println!(
                        "{fg_y}An entry for application '{fg_g}{tag}{fg_y}' already exists{rst}.\
                        \nNot overwriting it; new version is in {fg_b}{}{rst}.",
                        tmp_file.to_string_lossy()
                    );
                }
            }
            None => {
                println!("Saving {fg_g}{}{rst}", final_file.to_string_lossy());
                fs::rename(&tmp_file, &final_file)?;
            }
        }
        let exe_file = final_file.with_extension("exe");
        let wexe_file = repo.get_wexe_exe_path();
        if !wexe_file.exists() {
            eprintln!(
                "{fg_o}The wexe executable file {fg_y}{}{fg_o} is not installed; cannot copy it{rst}.",
                wexe_file.to_string_lossy()
            );
            return Ok(ExitCode::FAILURE);
        }
        if exe_file.exists() {
            println!(
                "{fg_y}Updating existing executable file {fg_c}{}{rst}.",
                exe_file.to_string_lossy()
            );
        } else {
            println!(
                "{fg_y}Creating application executable {fg_g}{}{rst}.",
                exe_file.to_string_lossy()
            );
            fs::copy(&wexe_file, &exe_file)?;
        }
        Ok(ExitCode::SUCCESS)
    }
}
