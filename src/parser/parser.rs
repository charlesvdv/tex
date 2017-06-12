use parser::context::ParserContext;
use parser::models::ParserElem;
use lexer::Lexer;
use parser::{ParsingInterpreter, InterpreterOutput, ParseResult};

use parser::comment::CommentInterpreter;
use parser::text::{TextInterpreter, EscapeCharInterpreter};
use parser::command::CommandInterpreter;


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    context: ParserContext,
    interpreters: Vec<Box<ParsingInterpreter>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input),
            context: ParserContext::new(),
            interpreters: vec![Box::new(CommentInterpreter::new()),
                               Box::new(TextInterpreter::new()),
                               Box::new(CommandInterpreter::new()),
                               Box::new(EscapeCharInterpreter::new())],
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<ParserElem>> {
        let mut result = vec![];
        'main: loop {
            let token = self.lexer.next();
            'int: for int in &self.interpreters {
                let out = int.interpret(token.elem(),
                                        &mut result,
                                        &mut self.lexer,
                                        &mut self.context)?;
                match out {
                    InterpreterOutput::Stop => break 'main,
                    InterpreterOutput::Matched => break 'int,
                    _ => continue,
                }
            }
        }
        Ok(result)
    }
}
