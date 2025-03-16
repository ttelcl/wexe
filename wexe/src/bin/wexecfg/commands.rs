use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::process::ExitCode;

use crate::args_buffer::ArgumentsBuffer;
use crate::help_central::HelpCentral;

pub trait Command {
    /// The primary name of the command. Defaults to the first item
    /// in the list returned by `name_and_aliases()`.
    fn name(&self) -> &str {
        self.name_and_aliases()[0]
    }
    /// The primary name and any aliases for the command.
    fn name_and_aliases(&self) -> &[&str];

    /// Execute the command. The success status is returned as an integer,
    /// usually 0 for success, 1 for soft failure (e.g. after showing a help
    /// message instead of actually doing something).
    fn execute(&self, args: &mut ArgumentsBuffer, commands: &CommandCollection) -> Result<ExitCode, Box<dyn Error>>;
}

pub struct CommandCollection {
    /// The commands in this collection, indexed and ordered by their primary name.
    commands: BTreeMap<String, Box<dyn Command>>,

    /// A map of all command names and aliases to their primary names.
    command_map: HashMap<String, String>,

    /// A reference to the help central.
    help_central: HelpCentral,
}

impl CommandCollection {
    /// Create a new, empty command collection.
    pub fn new() -> CommandCollection {
        let commands = CommandCollection {
            commands: BTreeMap::new(),
            command_map: HashMap::new(),
            help_central: HelpCentral::new(),
        };
        commands
    }

    /// Add a command to the collection (moving the command into it).
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        // First set up the command map, before moving the command into 'commands'.
        for name in command.name_and_aliases() {
            self.command_map
                .insert(name.to_string(), command.name().to_string());
        }
        self.commands.insert(command.name().to_string(), command);
    }

    /// Get a command by name or alias.
    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        match self.command_map.get(name) {
            Some(primary_name) => self.commands.get(primary_name),
            None => None,
        }
    }

    // /// Return a list of all commands in this collection as a newly allocated vector.
    // /// The commands are ordered by their primary name.
    // pub fn get_commands(&self) -> Vec<&Box<dyn Command>> {
    //     self.commands.values().collect()
    // }

    /// Print help for all commands.
    pub fn print_all_help(&self) -> Result<ExitCode, Box<dyn Error>> {
        self.help_central.print_all_help();
        Ok(ExitCode::FAILURE) // soft failure
    }

    /// Print help for a specific command.
    pub fn print_help_for(&self, command: &str) {
        self.help_central.print_help_for(command);
    }
}
