use lexer;
use parser::{ParseResult, ParserElem, ParserContext, ParsingInterpreter, InterpreterOutput,
             MacroDefParam};
use parser::command::SpecificCommandInterpreter;
use parser::errors::{ParserError, ErrorType};

pub struct MacroDefinitionInterpreter {}

impl MacroDefinitionInterpreter {
    pub fn new() -> Self {
        MacroDefinitionInterpreter {}
    }

    fn macro_param(&self, lexer: &lexer::Lexer) -> ParseResult<Vec<MacroDefParam>> {
        let mut params = vec![];
        let mut param_counter = 1;
        loop {
            let next_token = lexer.next();
            match next_token.elem() {
                &lexer::Elem::MacroParam => {
                    let number_token = lexer.next();
                    if let &lexer::Elem::Number(n) = number_token.elem() {
                        if param_counter == n {
                            param_counter += 1;
                            params.push(MacroDefParam::Parameter(n as u32));
                        } else {
                            return Err(ParserError::new_with_pos(ErrorType::MacroDefinition,
                                                                 *number_token.position(),
                                                                 &format!("Macro parameter \
                                                                          should increase \
                                                                          linearly and begin \
                                                                          by 1. (expected: {})",
                                                                          param_counter)));
                        }
                    } else {
                        return Err(ParserError::new_with_pos(ErrorType::MacroDefinition,
                                                             *number_token.position(),
                                                             "Number expected after a macro \
                                                             parameter"));
                    }
                }
                &lexer::Elem::Text(v) => params.push(MacroDefParam::Delimiter(v.into())),
                &lexer::Elem::Space => params.push(MacroDefParam::Delimiter(" ".into())),
                &lexer::Elem::BeginGroup => break,
                _ => {
                    return Err(ParserError::new_with_pos(ErrorType::MacroDefinition,
                                                         *next_token.position(),
                                                         "Impossible to parse this token as macro \
                                                         parameter or delimiter."))
                }
            }
        }

        Ok(params)
    }

    fn macro_body<'a>(&self, lexer: &lexer::Lexer<'a>) -> ParseResult<Vec<lexer::Elem<'a>>> {
        let mut elems = vec![];
        let mut group_counter = 0;
        loop {
            let (token, _) = lexer.next().consume();
            match token {
                lexer::Elem::EndGroup => {
                    if group_counter == 0 {
                        break;
                    } else {
                        group_counter -= 1;
                        elems.push(lexer::Elem::EndGroup);
                    }
                }
                lexer::Elem::BeginGroup => {
                    group_counter += 1;
                    elems.push(lexer::Elem::BeginGroup);
                }
                v => elems.push(v),
            }
        }
        Ok(elems)
    }
}

impl SpecificCommandInterpreter for MacroDefinitionInterpreter {
    fn interpret(&self,
                 command: &str,
                 _: &mut Vec<ParserElem>,
                 lexer: &mut lexer::Lexer,
                 ctx: &mut ParserContext)
                 -> ParseResult<InterpreterOutput> {
        if !["def", "edef"].contains(&command) {
            return Ok(InterpreterOutput::NoMatch);
        }

        // We should get the macro name.
        consume_spaces!(lexer);
        let next_token = lexer.next();

        let name;
        match next_token.elem() {
            &lexer::Elem::Command(v) => name = v,
            _ => {
                return Err(ParserError::new_with_pos(ErrorType::MacroDefinition,
                                                     *next_token.position(),
                                                     "Macro definition requires a name first."))
            }
        }
        consume_spaces!(lexer);

        // Next, we should have a BeginGroup to define the macro or a macro parameter
        // to define argument to the macro.
        let next_token = lexer.peek_next();
        let (params, body) = match next_token {
            lexer::Elem::BeginGroup => (None, self.macro_body(lexer)?),
            _ => (Some(self.macro_param(lexer)?), self.macro_body(lexer)?),
        };

        Ok(InterpreterOutput::Matched)
    }
}
