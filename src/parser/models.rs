pub enum ParserElem {
    Text(String),
    Command(Command),
}

pub struct Command {
    name: String,
    parameter: Option<Vec<CommandParam>>,
}

pub enum CommandParam {
    Parameter(u32),
    Delimiter(String),
}
