use std::collections::HashMap;
use lexer::Catcodes;
use parser::Command;

/// When defining a value or macros, we can assign the command to
/// differents scopes with for example: \global, \outer, ...
pub enum DefinitionScope {
    Outer,
    Global,
    Normal,
}

#[derive(Default)]
pub struct Group {
    catcode: Catcodes,
    macros: HashMap<String, Command>,
}

impl Group {
    pub fn add_macro_def(&mut self, name: &str, implementation: Command) {
        self.macros.insert(name.into(), implementation);
    }
}
