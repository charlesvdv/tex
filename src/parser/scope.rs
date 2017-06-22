use lexer::Catcodes;

/// Handle TeX groups.
pub struct Groups {
    default: DefaultScope,
    stack: Vec<ScopeHolder>,
}

impl Groups {
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

    /// Get all the scopes that we need to change according to the current default scope.
    fn get_scopes(&mut self) -> Vec<&mut ScopeHolder> {
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
struct ScopeHolder {
    catcodes: Catcodes,
}

impl ScopeHolder {
    pub fn new() -> ScopeHolder {
        ScopeHolder { catcodes: Catcodes::default() }
    }

    fn set_catcode(&mut self, code: usize, value: char) {
        self.catcodes.set_catcode(code, value);
    }
}
