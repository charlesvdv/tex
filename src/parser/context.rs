use parser::group::{Group, DefinitionScope};

#[derive(PartialEq, Eq)]
pub enum ContextMode {
    Normal,
    Math,
}
pub struct ParserContext {
    pub mode: ContextMode,
    scopes: Vec<Group>,
    def_scope: DefinitionScope,
}

impl ParserContext {
    pub fn new() -> Self {
        let global_scope = Group::default();

        ParserContext {
            mode: ContextMode::Normal,
            scopes: vec![global_scope],
            def_scope: DefinitionScope::Normal,
        }
    }

    pub fn get_default_scope(&self) -> &Group {
        match self.def_scope {
            DefinitionScope::Global => &self.scopes[0],
            DefinitionScope::Outer => {
                if self.scopes.len() == 1 {
                    &self.scopes[0]
                } else {
                    &self.scopes[self.scopes.len() - 1]
                }
            }
            _ => &self.scopes.last().unwrap()
        }
    }

    pub fn set_default_scope(&mut self, def: DefinitionScope) {
        self.def_scope = def;
    }

    pub fn add_scope(&mut self) {
        self.scopes.push(Group::default());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
