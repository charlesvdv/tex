use std::error::Error;
use std::fmt;

use lexer::Position;

#[derive(Debug)]
pub struct ParsingError {
    description: String,
    err_type: Option<ErrorType>,
    pos: Option<Position>,
}

impl ParsingError {
    pub fn new(description: &str) -> Self {
        ParsingError {
            description: description.into(),
            err_type: None,
            pos: None,
        }
    }

    pub fn position(&mut self, pos: Position) -> &mut Self {
        self.pos = Some(pos);
        self
    }

    pub fn error_type(&mut self, err_type: ErrorType) -> &mut Self {
        self.err_type = Some(err_type);
        self
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "Error")
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug)]
pub enum ErrorType {
    Catcodes,
}
