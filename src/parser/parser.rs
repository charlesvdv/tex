use lexer::Lexer;
use parser::{Context, ParsingResult, TeXToken, ParsingInterpreter, InterpreterOutput,
             InterpretersLauncher};

use parser::comment::CommentInterpreter;
use parser::text::TextInterpreter;
use parser::command::CommandInterpreter;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    ctx: Context,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input),
            ctx: Context::new(),
        }
    }

    pub fn parse(&mut self) -> ParsingResult<Vec<TeXToken>> {
        let handler = TopParserInterpreter::default();
        let mut out = vec![];

        match handler.launch_interpreters(
            &mut out,
            &mut self.lexer,
            &mut self.ctx,
        )? {
            Some(InterpreterOutput::Stop) => (),
            _ => {
                // TODO: return error.
            }
        }
        Ok(out)
    }
}

pub struct TopParserInterpreter {
    interpreters: Vec<Box<ParsingInterpreter>>,
}

impl TopParserInterpreter {
    pub fn new() -> Self {
        TopParserInterpreter { interpreters: vec![] }
    }
}

impl InterpretersLauncher for TopParserInterpreter {
    fn get_interpreters(&self) -> &Vec<Box<ParsingInterpreter>> {
        &self.interpreters
    }
}

impl Default for TopParserInterpreter {
    fn default() -> Self {
        TopParserInterpreter {
            interpreters: vec![
                Box::new(CommentInterpreter::new()),
                Box::new(CommandInterpreter::default()),
                Box::new(TextInterpreter::new()),
            ],
        }
    }
}
