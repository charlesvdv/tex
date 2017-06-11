use parser::{ParseResult, ParserElem, ParserContext, ParsingInterpreter, InterpreterOutput};
use lexer;

pub struct CommentInterpreter {}

impl CommentInterpreter {
    pub fn new() -> Self {
        CommentInterpreter {}
    }
}

impl ParsingInterpreter for CommentInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 result: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 _: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        let _ = match token {
            &lexer::Elem::Comment(_) => (),
            _ => return Ok(InterpreterOutput::NoMatch),
        };

        // We need to check if the comment is on a complete line or just
        // on the end of a line to see if we need to consume or not the linebreak.
        match result.last() {
            Some(v) => {
                if let &ParserElem::Text(ref t) = v {
                    if let Some(ch) = t.chars().last() {
                        if ch == '\n' {
                            // Consume the linebreak.
                            lexer.next();
                        }
                    }
                }
            }
            None => {
                lexer.next();
            }
        }

        Ok(InterpreterOutput::Matched)
    }
}
