use parser::{ParseResult, ParserElem, ParserContext, ParsingInterpreter, InterpreterOutput};
use lexer;

pub struct CommandInterpreter {}

impl CommandInterpreter {
    pub fn new() -> Self {
        CommandInterpreter {}
    }
}

impl ParsingInterpreter for CommandInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 _: &mut Vec<ParserElem>,
                 _: &mut lexer::Lexer,
                 _: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        let command = match token {
            &lexer::Elem::Command(v) => v,
            _ => return Ok(InterpreterOutput::NoMatch),
        };

        if command == "bye" {
            return Ok(InterpreterOutput::Stop);
        }

        Ok(InterpreterOutput::Matched)
    }
}
