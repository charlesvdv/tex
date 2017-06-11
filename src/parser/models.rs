#[derive(Debug)]
pub enum ParserElem {
    Text(String),
    Command(Command),
}

#[derive(Debug)]
pub struct Command {
    name: String,
    parameter: Option<Vec<CommandParam>>,
}

#[derive(Debug)]
pub enum CommandParam {
    Parameter(u32),
    Delimiter(String),
}
