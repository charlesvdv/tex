#[derive(PartialEq, Eq)]
pub enum ContextMode {
    Normal,
    Math,
}
pub struct ParserContext {
    pub mode: ContextMode,
}

impl ParserContext {
    pub fn new() -> Self {
        ParserContext { mode: ContextMode::Normal }
    }
}
