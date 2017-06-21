macro_rules! consume_spaces {
    ($lexer:expr) => (
        loop {
            match $lexer.peek_next() {
                Some(lexer::Token::Space) => (),
                _ => break,
            }
            // Consume the actual space.
            $lexer.next();
        }
    )
}

macro_rules! get_lex_token {
    ($lexer:expr) => (
        {
            let token = $lexer.next();
            if let None = token {
                return Err(ParsingError::new("Unexpected end of the data."));
            }
            token.unwrap().consume()
        }
    )
}

macro_rules! consume_lex_token {
    ($lexer:expr, $val:pat) => (
        {
            let (token, pos) = get_lex_token!($lexer);
            match token {
                $val => (),
                v => {
                    let msg = format!("Expected: {:?}, found: {:?}.", stringify!($val), v);
                    return Err(
                        ParsingError::new(&msg).position(pos),
                    );
                }
            }
        }
    )
}
