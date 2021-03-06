use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput};

/// Should be in the last interpreter as it takes a lot of differents tokens.
pub struct TextInterpreter {}

impl TextInterpreter {
    pub fn new() -> Self {
        TextInterpreter {}
    }

    fn init_text(&self, out: &mut Vec<TeXToken>) -> String {
        if out.is_empty() {
            return String::new();
        }

        // Check if the last token is text or not.
        let last_token = out.pop().unwrap();
        if let TeXToken::Paragraph(v) = last_token {
            return v;
        } else {
            out.push(last_token);
        }

        String::new()
    }
}

impl ParsingInterpreter for TextInterpreter {
    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::Text(_)) => true,
            Some(lexer::Token::LineBreak) => true,
            Some(lexer::Token::Space) => true,
            Some(lexer::Token::SpecialChar(_)) => true,
            _ => false,
        }
    }

    fn run(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        _: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        let mut txt = self.init_text(out);
        loop {
            lexer.reset_peek();
            if !self.matching(lexer) {
                break;
            }

            // It is safe to consume as we already checked if this token
            // was valid.
            match *lexer.next().unwrap().elem() {
                lexer::Token::Text(v) => txt.push_str(v),
                lexer::Token::LineBreak => {
                    if let Some(lexer::Token::LineBreak) = lexer.peek_next() {
                        // Consume line break.
                        lexer.next();

                        // Create a new paragraph and reset the `txt` variable.
                        out.push(TeXToken::Paragraph(txt));
                        txt = String::new();
                    } else {
                        txt.push(' ');
                    }
                }
                lexer::Token::Space => txt.push(' '),
                lexer::Token::SpecialChar(v) => txt.push(v),
                _ => unreachable!(),
            }
        }
        out.push(TeXToken::Paragraph(txt));

        Ok(None)
    }
}
