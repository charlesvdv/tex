use std::collections::HashMap;
use lexer::Catcodes;
use parser::MacroDefinition;

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
    macros: HashMap<String, MacroDefinition>,
}

impl Group {
    pub fn add_macro_def(&mut self, implementation: MacroDefinition) {
        self.macros.insert(implementation.name, implementation);
    }
}
