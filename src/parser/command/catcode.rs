use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, SpecificInterpreter, TeXToken, Context, ParsingResult,
             InterpreterOutput, ParsingError};
use parser::char::CharCodeInterpreter;

pub struct CatcodeCommandInterpreter {}

impl CatcodeCommandInterpreter {
    pub fn new() -> Self {
        CatcodeCommandInterpreter {}
    }
}

impl ParsingInterpreter for CatcodeCommandInterpreter {
    matching_command!("catcode");

    fn run(
        &self,
        _: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the command.
        lexer.next();

        // Get the char value.
        let char_interpreter = CharCodeInterpreter::new();
        if !char_interpreter.matching(lexer) {
            return Err(ParsingError::new(
                "Unable to match the char of the catcodes.",
            ));
        }
        let ch_value = char_interpreter.run(lexer, ctx)?;

        consume_spaces!(lexer);

        // Consume the `=` character.
        consume_lex_token!(lexer, lexer::Token::SpecialChar('='));

        consume_spaces!(lexer);

        // Get the category code.
        let (token, pos) = get_lex_token!(lexer);
        let category_code = match token {
            lexer::Token::Number(v) => v,
            v => {
                return Err(
                    ParsingError::new(&format!(
                        "Expected number for the category code, received: {:?}",
                        v
                    )).position(pos),
                );
            }
        };

        // Save the catcodes.
        ctx.groups_mut()
            .set_catcode(category_code as usize, ch_value);

        lexer.set_catcode(ctx.groups().get_catcode());
        Ok(None)
    }
}
