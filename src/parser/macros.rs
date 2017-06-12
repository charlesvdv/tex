macro_rules! consume_spaces {
    ($lexer:expr) => (
        loop {
            match $lexer.peek_next() {
                lexer::Elem::Space => (),
                _ => break,
            }
            // Consume the actual space.
            $lexer.next();
        }
    )
}
