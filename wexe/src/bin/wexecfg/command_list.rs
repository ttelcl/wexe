use std::error::Error;
use std::process::ExitCode;

use super::args_buffer::ArgumentsBuffer;
use super::commands::{Command, CommandCollection};
use super::wexe_repository::WexeRepository;
use super::wexe_repository::get_file_stamp;

use wexe::console_colors::*;

pub struct ListCommand {
    names: Vec<&'static str>,
}

struct ListCommandOptions {
    pub filter: Option<String>,
}

impl ListCommand {
    pub fn new() -> ListCommand {
        ListCommand {
            names: vec!["/list", "/l"],
        }
    }
}

impl ListCommandOptions {
    pub fn new() -> ListCommandOptions {
        ListCommandOptions { filter: None }
    }

    pub fn parse_args(&mut self, args: &mut ArgumentsBuffer) -> bool {
        while !args.is_empty() {
            let arg_key = args.get();
            match arg_key {
                "-m" => {
                    if args.remaining() < 2 {
                        eprintln!("{fg_o}Option {fg_y}-m{fg_o} requires an argument.{rst}",);
                        return false;
                    }
                    self.filter = Some(args.get_at(1).to_string());
                    args.skip(2);
                }
                _ => {
                    eprintln!("{fg_o}Unrecognized option: {fg_y}{:}{rst}.", arg_key);
                    return false;
                }
            }
        }
        true
    }
}

impl Command for ListCommand {
    fn name(&self) -> &str {
        self.names[0]
    }

    fn name_and_aliases(&self) -> &[&str] {
        self.names.as_ref()
    }

    fn execute(
        &self,
        _args: &mut ArgumentsBuffer,
        commands: &CommandCollection,
    ) -> Result<ExitCode, Box<dyn Error>> {
        let mut options = ListCommandOptions::new();
        if !options.parse_args(_args) {
            commands.print_help_for(self.name());
            return Ok(ExitCode::FAILURE);
        }
        println!("Registered applications{rst}:");
        let repo = WexeRepository::new();
        let apps = repo.get_entries();
        let title_tag = "Application";
        let title_stub = "Stub status";
        let title_target = "Target";
        println!(
            "{stl_u}{title_tag:<20} | {stl_u}{title_stub:<16} | {stl_u}{title_target:<40}.{rst}"
        );
        for app in apps.iter() {
            let tag = app.get_tag();
            if let Some(filter) = &options.filter {
                if !tag.contains(filter) {
                    continue;
                }
            }
            let target_exe_path = app.get_target_exe_path();
            let target_text = match target_exe_path {
                Some(path) => path.to_str().unwrap(),
                None => app.get_load_error().as_ref().unwrap(),
            };
            let style_tag;
            let style_target;
            if target_exe_path.is_none() {
                style_tag = format!("{stl_i}{fg_r}");
                style_target = format!("{stl_i}{fg_r}* Configuration Load Error: ");
            } else if !target_exe_path.as_ref().unwrap().exists() {
                style_tag = format!("{stl_i}{fg_r}");
                style_target = format!("{fg_r}{stl_i}* Target file missing: {rst}{stl_s}{fg_o}");
            } else {
                style_tag = format!("{fg_g}");
                style_target = format!("");
            }
            let stub_stamp = get_file_stamp(app.get_stub_exe_path());
            let stub_style: String;
            let stub_stamp_text: String = match stub_stamp {
                Some(stamp) => {
                    let stamp_text = stamp.format("%Y%m%d-%H%M%S").to_string();
                    stub_style = format!("{fg_y}");
                    format!("{:}", stamp_text)
                }
                None => {
                    stub_style = format!("{fg_o}{stl_i}");
                    format!("Stub missing",)
                }
            };
            println!(
                "{style_tag}{tag:<20}{rst} | {stub_style}{stub_stamp_text:<16}{rst} | {style_target}{target_text}{rst}"
            );
        }
        Ok(ExitCode::SUCCESS)
    }
}
