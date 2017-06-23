use super::*;

#[test]
fn test_normal_lexer() {
    let input = "foo bar \\{ -
\\bar \\foo{bar}
\\def \\foo #1v#2{bar}";

    let lexer = Lexer::new(input);

    let data = vec![
        LexerToken::new(Token::Text("foo bar "), Position::new(0, 0)),
        LexerToken::new(Token::EscapedChar('{'), Position::new(0, 8)),
        LexerToken::new(Token::Space, Position::new(0, 10)),
        LexerToken::new(Token::SpecialChar('-'), Position::new(0, 11)),
        LexerToken::new(Token::LineBreak, Position::new(0, 12)),
        LexerToken::new(Token::Command("bar"), Position::new(1, 0)),
        LexerToken::new(Token::Space, Position::new(1, 4)),
        LexerToken::new(Token::Command("foo"), Position::new(1, 5)),
        LexerToken::new(Token::BeginGroup, Position::new(1, 9)),
        LexerToken::new(Token::Text("bar"), Position::new(1, 10)),
        LexerToken::new(Token::EndGroup, Position::new(1, 13)),
        LexerToken::new(Token::LineBreak, Position::new(1, 14)),
        LexerToken::new(Token::Command("def"), Position::new(2, 0)),
        LexerToken::new(Token::Space, Position::new(2, 4)),
        LexerToken::new(Token::Command("foo"), Position::new(2, 5)),
        LexerToken::new(Token::Space, Position::new(2, 9)),
        LexerToken::new(Token::MacroParam, Position::new(2, 10)),
        LexerToken::new(Token::Number(1), Position::new(2, 11)),
        LexerToken::new(Token::Text("v"), Position::new(2, 12)),
        LexerToken::new(Token::MacroParam, Position::new(2, 13)),
        LexerToken::new(Token::Number(2), Position::new(2, 14)),
        LexerToken::new(Token::BeginGroup, Position::new(2, 15)),
        LexerToken::new(Token::Text("bar"), Position::new(2, 16)),
        LexerToken::new(Token::EndGroup, Position::new(2, 19)),
        LexerToken::new(Token::EndOfFile, Position::new(2, 20)),
    ];

    for d in data {
        assert_eq!(lexer.next(), Some(d));
    }
}

#[test]
fn test_peek_next() {
    let input = "\\foo a string\\bar
{a string enclosed by a group}";

    let lexer = Lexer::new(input);

    let data = vec![
        LexerToken::new(Token::Command("foo"), Position::new(0, 0)),
        LexerToken::new(Token::Text(" a string"), Position::new(0, 4)),
        LexerToken::new(Token::Command("bar"), Position::new(0, 13)),
        LexerToken::new(Token::LineBreak, Position::new(0, 17)),
        LexerToken::new(Token::BeginGroup, Position::new(1, 0)),
        LexerToken::new(
            Token::Text("a string enclosed by a group"),
            Position::new(1, 1),
        ),
        LexerToken::new(Token::EndGroup, Position::new(1, 29)),
        LexerToken::new(Token::EndOfFile, Position::new(1, 30)),
    ];

    for d in &data {
        assert_eq!(lexer.peek_next().unwrap(), *d.elem());
    }

    lexer.reset_peek();

    for d in &data {
        assert_eq!(lexer.peek_next().unwrap(), *d.elem());
    }

    for d in data {
        assert_eq!(lexer.next().unwrap(), d);
    }
}

#[test]
fn test_next_and_peek() {
    let input = "\\foo a string\\bar
{a string enclosed by a group}";

    let lexer = Lexer::new(input);

    let data = vec![
        LexerToken::new(Token::Command("foo"), Position::new(0, 0)),
        LexerToken::new(Token::Text(" a string"), Position::new(0, 4)),
        LexerToken::new(Token::Command("bar"), Position::new(0, 13)),
        LexerToken::new(Token::LineBreak, Position::new(0, 17)),
        LexerToken::new(Token::BeginGroup, Position::new(1, 0)),
        LexerToken::new(
            Token::Text("a string enclosed by a group"),
            Position::new(1, 1),
        ),
        LexerToken::new(Token::EndGroup, Position::new(1, 29)),
        LexerToken::new(Token::EndOfFile, Position::new(1, 30)),
    ];

    assert_eq!(lexer.peek_next().unwrap(), *data[0].elem());
    assert_eq!(lexer.peek_next().unwrap(), *data[1].elem());

    assert_eq!(lexer.next().unwrap(), data[0]);

    assert_eq!(lexer.peek_next().unwrap(), *data[1].elem());
    assert_eq!(lexer.peek_next().unwrap(), *data[2].elem());

    lexer.reset_peek();

    assert_eq!(lexer.peek_next().unwrap(), *data[1].elem());
    assert_eq!(lexer.peek_next().unwrap(), *data[2].elem());
}
