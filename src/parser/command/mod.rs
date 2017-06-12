use parser::{ParseResult, ParserElem, ParserContext, ParsingInterpreter, InterpreterOutput};
use lexer;

pub mod def;

pub trait SpecificCommandInterpreter {
    fn interpret(&self,
                 command: &str,
                 result: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 ctx: &mut ParserContext)
                 -> ParseResult<InterpreterOutput>;
}

pub struct CommandInterpreter {
    interpreters: Vec<Box<SpecificCommandInterpreter>>,
}

impl CommandInterpreter {
    pub fn new() -> Self {
        CommandInterpreter { interpreters: vec![Box::new(def::MacroDefinitionInterpreter::new())] }
    }
}

impl ParsingInterpreter for CommandInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 result: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 ctx: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        let command = match token {
            &lexer::Elem::Command(v) => v,
            _ => return Ok(InterpreterOutput::NoMatch),
        };

        if command == "bye" {
            return Ok(InterpreterOutput::Stop);
        }

        for int in &self.interpreters {
            let out = int.interpret(command, result, lexer, ctx)?;
            match out {
                InterpreterOutput::Matched => break,
                InterpreterOutput::Stop => return Ok(InterpreterOutput::Stop),
                _ => continue,
            }
        }

        Ok(InterpreterOutput::Matched)
    }
}
