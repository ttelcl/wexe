#![allow(dead_code)]

use wexe::console_colors::*;

pub struct CommandHelp {
    pub command: String,
    pub synopsis: String,
    pub description: String,
    pub options: Vec<String>,
}

impl CommandHelp {
    pub fn print(&self) {
        println!("{fg_o}wexecfg {}{rst}", self.synopsis);
        println!("    {}{rst}", self.description);
        for opt in self.options.iter() {
            println!("    {}{rst}", opt);
        }
    }
}

fn init_help() -> Vec<CommandHelp> {
    let mut help = Vec::new();
    help.push(CommandHelp {
        command: "/help".into(),
        synopsis: format!("{fg_y}/help{rst}"),
        description: "Show help for all commands".into(),
        options: Vec::new(),
    });
    help.push(CommandHelp {
        command: "/list".into(),
        synopsis: format!("{fg_y}/list{rst} [{fg_g}-m {fg_c}{stl_i}filter{rst}]"),
        description: "List all configured applications".into(),
        options: vec![format!(
            "{fg_g}-m {fg_c}{stl_i}filter{rst}       If given, only \
                list applications with the {fg_c}{stl_i}filter{rst} string in their name."
        )],
    });
    help.push(CommandHelp {
        command: "/wrap".into(),
        synopsis: format!(
            "{fg_y}/wrap {fg_g}-x {fg_c}{stl_i}target.exe{rst} [{fg_g}-n {fg_c}{stl_i}name{rst}] \
            {{{fg_g}-a {fg_c}{stl_i}argument{rst}}} \
            {{{fg_g}-p {fg_c}{stl_i}path{rst}}} \
            [{fg_g}-F{rst}]"),
        description: format!(
            "Create a new application for the target executable. Creates a configuration file and a stub \
            executable.\
            \n    {fg_W}\u{2022} {stl_i}If the application configuration already exists, a candidate configuration \
            file ({fg_o}app.toml.tmp{fg_W})\
            \n      is created instead, unless {fg_g}-F{fg_W} is specified{rst}.\
            \n    {fg_W}\u{2022} {stl_i}If the stub executable already exists, it is not replaced.{rst}."
        )
        .into(),
        options: vec![format!(
            "{fg_g}-x {fg_c}{stl_i}target.exe{rst}   The path to the target executable. Also implies \
            the name of the application, unless {fg_g}-n{rst} is given."
        ),
        format!(
            "{fg_g}-n {fg_c}{stl_i}name{rst}         If given, overrides the name of the application."
        ),
        format!(
            "{fg_g}-a {fg_c}{stl_i}argument{rst}     ({stl_i}repeatable{rst}) Extra command-line argument to prepend."
        ),
        format!(
            "{fg_g}-p {fg_c}{stl_i}path{rst}         ({stl_i}repeatable{rst}) Extra path to prepend to PATH."
        ),
        format!(
            "{fg_g}-F{rst}              ('{stl_i}Force{rst}') If the configuration file already exists, overwrite it instead of \
            creating a candidate."
        )],
    });
    help.push(CommandHelp {
        command: "/fix".into(),
        synopsis: format!("{fg_y}/fix{fg_W} [{fg_g}-all{fg_W}|{fg_c}{stl_i}app-name{rst}]"),
        description: format!(
            "Recreate the application stub for the specified application or all applications."
        )
        .into(),
        options: vec![
            format!("{fg_g}-all{fg_W}            Update all application stubs."),
            format!("{fg_c}{stl_i}app-name{rst}        Update {fg_c}{stl_i}app-name{rst} only."),
        ],
    });
    help.push(CommandHelp {
        command: "/install".into(),
        synopsis: format!("{fg_y}/install{rst} [{fg_g}-wexe{rst}]"),
        description: format!(
            "Install the currently running {fg_o}wexecfg{rst} executable (and \
            optionally the associated {fg_o}wexe{rst}\n    executable) into the wexe install folder.\
            \n    {fg_W}\u{2022} {stl_i}Fails if invoked on the installed {fg_o}wexecfg{fg_W} executable. \
            Use the debug or release build output instead{rst}.\
            \n    {fg_W}\u{2022} {stl_i}Does not replace stub executables (copies of {fg_o}wexe{fg_W}); use \
            {fg_o}wexecfg {fg_y}/fix{fg_W} for that{rst}."
        )
        .into(),
        options: vec![format!(
            "{fg_g}-wexe{rst}           If given, also copies {fg_o}wexe.exe{rst} to the installation folder."
        )],
    });
    help
}

pub struct HelpCentral {
    commands: Vec<CommandHelp>,
}

impl HelpCentral {
    pub fn new() -> HelpCentral {
        let help = init_help();
        HelpCentral { commands: help }
    }

    pub fn add_help(&mut self, command: CommandHelp) {
        self.commands.push(command);
    }

    pub fn get_help(&self, command: &str) -> Option<&CommandHelp> {
        for cmd in self.commands.iter() {
            if cmd.command == command {
                return Some(cmd);
            }
        }
        None
    }

    pub fn print_help_for(&self, command: &str) {
        match self.get_help(command) {
            Some(help) => {
                help.print();
            }
            None => {
                println!(
                    "{fg_o}No help available for command{rst} '{fg_y}{}{rst}'.",
                    command
                );
            }
        }
    }

    pub fn print_all_help(&self) {
        for help in self.commands.iter() {
            help.print();
        }
    }
}
