use parser::{ParseResult, ParserElem, ParserContext, ParsingInterpreter, InterpreterOutput};
use parser::context::ContextMode;
use parser::errors::{ParserError, ErrorType};
use lexer;

// Handles also the LineBreak for now.
pub struct TextInterpreter {}

impl TextInterpreter {
    pub fn new() -> Self {
        TextInterpreter {}
    }
}

impl ParsingInterpreter for TextInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 result: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 _: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        let txt = match token {
            &lexer::Elem::Text(v) => v,
            &lexer::Elem::LineBreak => "\n",
            _ => return Ok(InterpreterOutput::NoMatch),
        };
        let mut txt = String::from(txt);

        loop {
            {
                let next = lexer.peek_next();
                match next {
                    &lexer::Elem::Text(v) => txt.push_str(v),
                    &lexer::Elem::LineBreak => txt.push('\n'),
                    _ => break,
                }
            }
            // Consume the text we just added.
            lexer.next();
        }

        result.push(ParserElem::Text(txt));

        Ok(InterpreterOutput::Matched)
    }
}

pub struct EscapeCharInterpreter {}

impl EscapeCharInterpreter {
    pub fn new() -> Self {
        EscapeCharInterpreter {}
    }
}

impl ParsingInterpreter for EscapeCharInterpreter {
    fn interpret(&self,
                 token: &lexer::Elem,
                 result: &mut Vec<ParserElem>,
                 _: &mut lexer::Lexer,
                 ctx: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        let ch = match token {
            &lexer::Elem::EscapedChar(v) => v,
            _ => return Ok(InterpreterOutput::NoMatch),
        };

        // TODO: if ~ is in front of a letter, it creates a specific accent.
        let normal_ch_escape = ['%', '&', '$', '~', '_'];
        if normal_ch_escape.contains(&ch) {
            result.push(ParserElem::Text(ch.to_string()))
        }

        if ['{', '}'].contains(&ch) {
            if ctx.mode == ContextMode::Math {
                result.push(ParserElem::Text(ch.to_string()))
            } else {
                return Err(ParserError::new(ErrorType::EscapedChar(ch),
                                            "`{` or `}` can't be escaped without \
                                            being inside math mode."));
            }
        }

        if ch == '^' {
            return Err(ParserError::new(ErrorType::EscapedChar(ch), "escaping ^ is not allowed."));
        }

        Ok(InterpreterOutput::Matched)
    }
}
