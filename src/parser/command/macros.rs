macro_rules! matching_command {
    ($command:expr) => (
        fn matching(&self, lexer: &LexTokenIterator) -> bool {
            match lexer.peek_next() {
                Some(lexer::Token::Command($command)) => true,
                _ => false,
            }
        }
    );
    ( $( $command:expr ),* ) => (
        fn matching(&self, lexer: &LexTokenIterator) -> bool {
            match lexer.peek_next() {
                $(
                    Some(lexer::Token::Command($command)) => true,
                )*
                _ => false,
            }
        }
    );
}
