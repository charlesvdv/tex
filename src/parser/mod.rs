use lexer;

pub mod errors;
mod models;
mod context;
mod parser;

// Parsing interpreter.
pub mod comment;
pub mod text;
pub mod command;

pub use self::parser::*;
pub use self::models::*;
pub use self::context::ParserContext;

type ParseResult<T> = Result<T, errors::ParserError>;

pub trait ParsingInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 result: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 ctx: &mut ParserContext)
                 -> ParseResult<InterpreterOutput>;
}

pub enum InterpreterOutput {
    NoMatch,
    Matched,
    Stop,
}
