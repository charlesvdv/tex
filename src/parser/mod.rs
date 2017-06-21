use lexer::LexTokenIterator;
pub mod parser;
pub mod context;
pub mod errors;
pub mod models;

// Interpreters.
pub mod comment;
pub mod text;
pub mod command;

pub use self::errors::ParsingError;
pub use self::context::Context;
pub use self::models::*;
pub use self::parser::Parser;

type ParsingResult<T> = Result<T, ParsingError>;

pub trait ParsingInterpreter {
    /// Check if the interpreter will match for the input.
    fn matching(&self, lexer: &LexTokenIterator) -> bool;

    /// Execute the logic inside the interpreter.
    ///
    /// Run the interpreter. We should check if the interpreter match before
    /// calling this function.
    fn run(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>>;
}

/// Give various information about the state of the parsing.
pub enum InterpreterOutput {
    /// The parser should stop.
    Stop,
}

pub trait InterpretersLauncher {
    fn get_interpreters(&self) -> &Vec<Box<ParsingInterpreter>>;

    /// Only launch one interpreter and then return with the result.
    fn launch_interpreters_once(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        let mut matched = false;

        for interpreter in self.get_interpreters() {
            // Reset peek each time we check for a match.
            lexer.reset_peek();
            if !interpreter.matching(&*lexer) {
                continue;
            }
            // Reset peek after we have a match.
            lexer.reset_peek();

            matched = true;
            match interpreter.run(out, lexer, ctx)? {
                Some(InterpreterOutput::Stop) => return Ok(Some(InterpreterOutput::Stop)),
                None => continue,
            }
        }

        if !matched {
            return Err(ParsingError::new(&format!(
                "No interpreter matched for token: {:?}",
                lexer.peek_next()
            )));
        }
        Ok(None)
    }

    /// Launch interpreters until the end of an input.
    fn launch_interpreters(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        loop {
            match self.launch_interpreters_once(out, lexer, ctx)? {
                Some(InterpreterOutput::Stop) => return Ok(Some(InterpreterOutput::Stop)),
                None => continue,
            }
        }
    }
}
