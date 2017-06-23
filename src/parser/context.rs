use parser::scope::Groups;

pub struct Context<'a> {
    groups: Groups<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context { groups: Groups::new() }
    }

    pub fn groups(&self) -> &Groups<'a> {
        &self.groups
    }

    pub fn groups_mut(&mut self) -> &mut Groups<'a> {
        &mut self.groups
    }
}
