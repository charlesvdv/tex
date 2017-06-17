use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput,
             InterpretersLauncher};

#[macro_use]
mod macros;

// Interpreters.
pub mod quit;

pub struct CommandInterpreter {
    interpreters: Vec<Box<ParsingInterpreter>>,
}

/// Group every interpreter which handles commands.
impl CommandInterpreter {
    pub fn new() -> Self {
        CommandInterpreter { interpreters: vec![] }
    }
}

impl Default for CommandInterpreter {
    fn default() -> Self {
        CommandInterpreter { interpreters: vec![Box::new(quit::QuitCommandInterpreter::new())] }
    }
}

impl ParsingInterpreter for CommandInterpreter {
    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::Command(_)) => true,
            _ => false,
        }
    }

    fn run(
        &self,
        out: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        lexer.reset_peek();
        self.launch_interpreters_once(out, lexer, ctx)
    }
}

impl InterpretersLauncher for CommandInterpreter {
    fn get_interpreters(&self) -> &Vec<Box<ParsingInterpreter>> {
        &self.interpreters
    }
}
