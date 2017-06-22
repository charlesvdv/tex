use lexer;
use lexer::LexTokenIterator;
use parser::{ParsingInterpreter, TeXToken, Context, ParsingResult, InterpreterOutput};

pub struct BeginGroupInterpreter {}

impl BeginGroupInterpreter {
    pub fn new() -> Self {
        BeginGroupInterpreter {}
    }
}

impl ParsingInterpreter for BeginGroupInterpreter {
    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::BeginGroup) => true,
            _ => false,
        }
    }

    fn run(
        &self,
        _: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the next token.
        lexer.next();

        ctx.groups_mut().push_scope();
        Ok(None)
    }
}

pub struct EndGroupInterpreter {}

impl EndGroupInterpreter {
    pub fn new() -> Self {
        EndGroupInterpreter {}
    }
}

impl ParsingInterpreter for EndGroupInterpreter {
    fn matching(&self, lexer: &LexTokenIterator) -> bool {
        match lexer.peek_next() {
            Some(lexer::Token::EndGroup) => true,
            _ => false,
        }
    }

    fn run(
        &self,
        _: &mut Vec<TeXToken>,
        lexer: &mut LexTokenIterator,
        ctx: &mut Context,
    ) -> ParsingResult<Option<InterpreterOutput>> {
        // Consume the next token.
        lexer.next();

        ctx.groups_mut().pop_scope();

        // Reset new catcodes after the last group was popped.
        lexer.set_catcode(ctx.groups().get_catcode());
        Ok(None)
    }
}
