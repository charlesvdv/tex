use lexer::Catcodes;
use parser::MacroDefinition;

/// Handle TeX groups.
pub struct Groups<'a> {
    default: DefaultScope,
    stack: Vec<ScopeHolder<'a>>,
}

impl<'a> Groups<'a> {
    pub fn new() -> Self {
        Groups {
            default: DefaultScope::Normal,
            // Add the global scope when creating the stack.
            stack: vec![ScopeHolder::new()],
        }
    }

    pub fn push_scope(&mut self) {
        let new_scope = self.stack.last().unwrap().clone();
        self.stack.push(new_scope);
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    pub fn set_default_scope(&mut self, default_scope: DefaultScope) {
        self.default = default_scope;
    }

    pub fn reset_default_scope(&mut self) {
        self.default = DefaultScope::Normal;
    }

    pub fn get_default_scope(&self) -> &DefaultScope {
        &self.default
    }

    pub fn get_catcode(&self) -> Catcodes {
        self.stack.last().unwrap().catcodes.clone()
    }

    pub fn set_catcode(&mut self, code: usize, value: char) {
        for scope in self.get_scopes() {
            scope.set_catcode(code, value);
        }
    }

    pub fn add_macro_definition(&mut self, def: MacroDefinition<'a>) {
        let mut scopes = self.get_scopes();
        scopes.last_mut().unwrap().macro_defs.push(def);
    }

    /// Get all the scopes that we need to change according to the current default scope.
    fn get_scopes(&mut self) -> Vec<&mut ScopeHolder<'a>> {
        match self.default {
            // Should be safe to unwrap because we can't have a state where the global scope
            // is not present.
            DefaultScope::Normal => vec![self.stack.last_mut().unwrap()],
            DefaultScope::Outer => {
                // Check if we only have the global scope or not.
                if self.stack.len() < 2 {
                    return vec![self.stack.last_mut().unwrap()];
                } else {
                    self.stack.iter_mut().rev().take(2).collect()
                }
            }
            DefaultScope::Global => self.stack.iter_mut().collect(),
        }
    }
}

pub enum DefaultScope {
    Global,
    Outer,
    Normal,
}

/// Hold information about one scope.
#[derive(Clone)]
struct ScopeHolder<'a> {
    catcodes: Catcodes,
    macro_defs: Vec<MacroDefinition<'a>>,
}

impl<'a> ScopeHolder<'a> {
    pub fn new() -> ScopeHolder<'a> {
        ScopeHolder {
            catcodes: Catcodes::default(),
            macro_defs: vec![],
        }
    }

    fn set_catcode(&mut self, code: usize, value: char) {
        self.catcodes.set_catcode(code, value);
    }
}
