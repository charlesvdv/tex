use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput};

pub struct QuitCommandInterpreter {}

impl QuitCommandInterpreter {
    pub fn new() -> Self {
        QuitCommandInterpreter {}
    }
}

impl ParsingInterpreter for QuitCommandInterpreter {
    matching_command!("bye");

    fn run(
        &self,
        _: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        _: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the command.
        lexer.next();
        Ok(Some(InterpreterOutput::Stop))
    }
}
