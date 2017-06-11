use std::fmt;
use std::fmt::Display;
use std::error::Error;
use lexer::Position;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    EscapedChar(char),
}

#[derive(Debug)]
pub struct ParserError {
    err_type: ErrorType,
    description: String,
    pos: Option<Position>,
}

impl ParserError {
    pub fn new(err_type: ErrorType, description: &str) -> Self {
        ParserError {
            err_type,
            description: description.into(),
            pos: None,
        }
    }

    pub fn new_with_pos(err_type: ErrorType, pos: Position, description: &str) -> Self {
        ParserError {
            err_type,
            description: description.into(),
            pos: Some(pos),
        }
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = Some(pos);
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.description
    }
}
