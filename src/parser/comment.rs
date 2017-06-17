use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput};

pub struct CommentInterpreter {}

impl CommentInterpreter {
    pub fn new() -> Self {
        CommentInterpreter {}
    }
}

impl ParsingInterpreter for CommentInterpreter {
    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::Comment(_)) => true,
            _ => false,
        }
    }

    fn run(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        _: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the comment.
        lexer.next();

        // We need to check if the comment is on a complete line or just
        // on the end of a line to see if we need to consume or not the linebreak.
        match out.last() {
            Some(v) => {
                if let &TeXToken::Text(ref t) = v {
                    if let Some(ch) = t.chars().last() {
                        if ch == '\n' {
                            // Consume the linebreak.
                            lexer.next();
                        }
                    }
                }
            }
            // As we don't have any data yet, this means the comment is on the top of
            // the file.
            None => {
                lexer.next();
            }
        }

        Ok(None)
    }
}
