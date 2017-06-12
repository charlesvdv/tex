#[derive(Debug)]
pub enum ParserElem {
    Text(String),
    Command(Command),
}

#[derive(Debug)]
pub struct Command {
    name: String,
    parameter: Option<Vec<MacroDefParam>>,
}

#[derive(Debug)]
pub enum MacroDefParam {
    Parameter(u32),
    Delimiter(String),
}
