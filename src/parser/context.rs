use parser::scope::Groups;

pub struct Context {
    groups: Groups,
}

impl Context {
    pub fn new() -> Self {
        Context { groups: Groups::new() }
    }

    pub fn groups(&self) -> &Groups {
        &self.groups
    }

    pub fn groups_mut(&mut self) -> &mut Groups {
        &mut self.groups
    }
}
