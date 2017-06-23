use lexer;

/// Define high level structure that the parser will output.
#[derive(Debug)]
pub enum TeXToken {
    Paragraph(String),
    BS,
}

#[derive(Debug, Clone)]
pub struct MacroDefinition<'a> {
    name: &'a str,
    params: Vec<MacroParameter>,
    body: Vec<lexer::Token<'a>>,
}

impl<'a> MacroDefinition<'a> {
    pub fn new(name: &'a str, params: Vec<MacroParameter>, body: Vec<lexer::Token<'a>>) -> Self {
        MacroDefinition { name, params, body }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn params(&self) -> &Vec<MacroParameter> {
        &self.params
    }

    pub fn body(&self) -> &Vec<lexer::Token<'a>> {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub enum MacroParameter {
    Parameter,
    Delimiter(String),
}
