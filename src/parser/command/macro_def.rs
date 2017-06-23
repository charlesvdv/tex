use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput,
             ParsingError, MacroParameter, MacroDefinition};

pub struct MacroDefCommandInterpreter {}

impl MacroDefCommandInterpreter {
    pub fn new() -> Self {
        MacroDefCommandInterpreter {}
    }

    fn macro_params(&self, lexer: &lexer::LexTokenIterator) -> ParsingResult<Vec<MacroParameter>> {
        let mut params = vec![];
        let mut param_counter = 1;
        loop {
            let (token, pos) = get_lex_token!(lexer);
            match token {
                lexer::Token::MacroParam => {
                    let (number_token, pos) = get_lex_token!(lexer);
                    match number_token {
                        lexer::Token::Number(v) => {
                            if param_counter == v {
                                param_counter += 1;
                                params.push(MacroParameter::Parameter);
                            } else {
                                return Err(ParsingError::new(&format!(
                                    "Macro parameter should increase linearly and begin by 1. \
                                     (expected: {})",
                                    param_counter
                                )));
                            }
                        }
                        _ => {
                            return Err(
                                ParsingError::new("Number expected after a macro parameter")
                                    .position(pos),
                            );
                        }
                    }
                }
                lexer::Token::Text(v) => params.push(MacroParameter::Delimiter(v.into())),
                lexer::Token::Space => params.push(MacroParameter::Delimiter(" ".into())),
                lexer::Token::BeginGroup => break,
                v => {
                    return Err(
                        ParsingError::new(&format!(
                            "Unexpected macro parameter or macro delimiter. (Found: {:?})",
                            v
                        )).position(pos),
                    );
                }
            }
        }
        Ok(params)
    }

    fn body<'a>(&self, lexer: &mut LexTokenIterator<'a>) -> ParsingResult<Vec<lexer::Token<'a>>> {
        let mut tokens = vec![];
        let mut group_counter = 0;

        loop {
            let (token, _) = get_lex_token!(lexer);
            match token {
                lexer::Token::BeginGroup => {
                    group_counter += 1;
                }
                lexer::Token::EndGroup => {
                    if group_counter == 0 {
                        break;
                    }
                    group_counter -= 1;
                }
                _ => (),
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

impl ParsingInterpreter for MacroDefCommandInterpreter {
    matching_command!("def", "edef");

    fn run<'a>(
        &self,
        _: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator<'a>,
        ctx: &mut Context<'a>,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the command.
        lexer.next();

        consume_spaces!(lexer);

        // Get the macro name.
        let (token, pos) = get_lex_token!(lexer);
        let name = match token {
            lexer::Token::Command(v) => v,
            v => {
                return Err(
                    ParsingError::new(&format!(
                        "Expected command describing the macro name, received: {:?}",
                        v
                    )).position(pos),
                )
            }
        };

        let params = self.macro_params(lexer)?;
        let body = self.body(lexer)?;

        let def = MacroDefinition::new(name, params, body);

        // TODO: expand when we have `edef`.
        ctx.groups_mut().add_macro_definition(def);

        Ok(None)
    }
}
