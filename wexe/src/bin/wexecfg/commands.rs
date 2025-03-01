
#![allow(dead_code)]

use std::error::Error;

pub trait Command {
    /// The primary name of the command. Defaults to the first item
    /// in the list returned by `name_and_aliases()`.
    fn name(&self) -> &str {
        self.name_and_aliases()[0]
    }
    /// The primary name and any aliases for the command.
    fn name_and_aliases(&self) -> &Vec<&str>;
    /// Execute the command. The success status is returned as an integer,
    /// usually 0 for success, 1 for soft failure (e.g. after showing a help
    /// message instead of actually doing something).
    fn execute(&self, args: &Vec<&str>) -> Result<i32, Box<dyn Error>>;
}
