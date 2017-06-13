use lexer;

/// Represents an element returned by the parser.
#[derive(Debug)]
pub enum ParserElem {
    Text(String),
    Command(Command),
}

/// Represents TeX primitives that we don't interpret.
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub parameter: Option<Vec<ParserElem>>,
}

/// Represents a definition of a macro.
pub struct MacroDefinition<'a> {
    pub name: String,
    pub parameters: Option<Vec<MacroDefParam>>,
    pub body: MacroDefBody<'a>,
}

/// A macro can be defined by two ways:
///
/// - `def` where the macro is evaluated when the macro is called (Raw)
/// - `edef` where macro is evaluated at definition (Expanded)
#[derive(Debug)]
pub enum MacroDefBody<'a> {
    Expanded(ParserElem),
    Raw(lexer::Elem<'a>),
}

#[derive(Debug)]
pub enum MacroDefParam {
    Parameter(u32),
    Delimiter(String),
}
