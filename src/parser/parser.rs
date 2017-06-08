use parser::context::ParserContext;
use parser::errors::ParsingError;
use parser::models::ParserElem;
use lexer::{Lexer, LexerElem, Elem};

type ParseResult<T> = Result<T, ParsingError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    context: ParserContext,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input),
            context: ParserContext::new(),
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<ParserElem>> {
        let mut result = vec![];
        loop {
            let token = self.lexer.next();
            match token.elem() {
                &Elem::Text(v) => result.push(ParserElem::Text(v.into())),
                &Elem::EscapedChar(v) => result.push(ParserElem::Text(v.to_string())),
                &Elem::LineBreak => result.push(ParserElem::Text("\n".into())),
                &Elem::Comment(_) => {
                    // We need to check if the comment is on a complete line or just
                    // on the end of a line to see if we need to consume or not the linebreak.
                    match result.last() {
                        Some(v) => {
                            if let &ParserElem::Text(ref t) = v {
                                if let Some(ch) = t.chars().last() {
                                    if ch == '\n' {
                                        // Consume the linebreak.
                                        self.lexer.next();
                                    }
                                }
                            }
                        }
                        None => {
                            self.lexer.next();
                        }
                    }
                }
                &Elem::Command(v) => {
                    if v == "bye" {
                        return Ok(result);
                    }
                    continue;
                }
                _ => continue,
            }
        }

        Ok(result)
    }
}
