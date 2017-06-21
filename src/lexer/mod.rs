mod models;
mod catcodes;
mod streamer;
mod lexer;
#[cfg(test)]
mod tests;

use self::streamer::ZeroCopyStreamer;

pub use self::lexer::Lexer;
pub use self::models::*;
pub use self::catcodes::Catcodes;

pub trait LexTokenIterator<'a> {
    fn next(&self) -> Option<LexerToken<'a>>;
    fn peek_next(&self) -> Option<Token<'a>>;
    fn reset_peek(&self);
    fn set_catcode(&mut self, catcodes: Catcodes);
}
