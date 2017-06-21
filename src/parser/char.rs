use lexer;
use lexer::LexTokenIterator;
use parser::{SpecificInterpreter, Context, ParsingResult, ParsingError};

// TODO: handle special characters defined with a command.
pub struct CharCodeInterpreter {}

impl CharCodeInterpreter {
    pub fn new() -> Self {
        CharCodeInterpreter {}
    }

    fn handle_single_char(&self, lexer: &mut LexTokenIterator) -> ParsingResult<char> {
        let (token, pos) = get_lex_token!(lexer);

        match token {
            lexer::Token::Text(v) => {
                // We should only have a char.
                if v.len() != 1 {
                    return Err(
                        ParsingError::new("Char definition should only have one char.")
                            .position(pos),
                    );
                }
                Ok(v.chars().nth(0).unwrap())
            }
            lexer::Token::EscapedChar(v) => Ok(v),
            v => Err(
                ParsingError::new(&format!("Unexpected token for a char, {:?}", v)).position(pos),
            ),
        }
    }
}

impl SpecificInterpreter for CharCodeInterpreter {
    type Out = char;

    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::SpecialChar('`')) => true,
            Some(lexer::Token::Number(_)) => true,
            _ => false,
        }
    }

    fn run(&self, lexer: &mut LexTokenIterator, _: &mut Context) -> ParsingResult<Self::Out> {
        let (token, pos) = get_lex_token!(lexer);
        match token {
            lexer::Token::SpecialChar('`') => {
                // There is 2 possibilities:
                // We can have a single char or an escaped char.
                self.handle_single_char(lexer)
            }
            lexer::Token::Number(v) => {
                // Number should only be ASCII.
                if v < 0 || v > 255 {
                    return Err(
                        ParsingError::new("Char number should only be ASCII.").position(pos),
                    );
                }
                Ok(char::from(v as u8))
            }
            _ => unreachable!(),
        }
    }
}
