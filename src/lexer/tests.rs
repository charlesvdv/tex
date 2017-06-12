use super::*;

#[test]
fn test_normal_lexer() {
    let input = "foo bar \\{ -
\\bar \\foo{bar}
\\def \\foo #1v#2{bar}";

    let lexer = Lexer::new(input);

    let data = vec![LexerElem::new(Elem::Text("foo bar "), Position::new(0, 0)),
                    LexerElem::new(Elem::EscapedChar('{'), Position::new(0, 8)),
                    LexerElem::new(Elem::Space, Position::new(0, 10)),
                    LexerElem::new(Elem::SpecialChar('-'), Position::new(0, 11)),
                    LexerElem::new(Elem::LineBreak, Position::new(0, 12)),
                    LexerElem::new(Elem::Command("bar"), Position::new(1, 0)),
                    LexerElem::new(Elem::Space, Position::new(1, 4)),
                    LexerElem::new(Elem::Command("foo"), Position::new(1, 5)),
                    LexerElem::new(Elem::BeginGroup, Position::new(1, 9)),
                    LexerElem::new(Elem::Text("bar"), Position::new(1, 10)),
                    LexerElem::new(Elem::EndGroup, Position::new(1, 13)),
                    LexerElem::new(Elem::LineBreak, Position::new(1, 14)),
                    LexerElem::new(Elem::Command("def"), Position::new(2, 0)),
                    LexerElem::new(Elem::Space, Position::new(2, 4)),
                    LexerElem::new(Elem::Command("foo"), Position::new(2, 5)),
                    LexerElem::new(Elem::Space, Position::new(2, 9)),
                    LexerElem::new(Elem::MacroParam, Position::new(2, 10)),
                    LexerElem::new(Elem::Number(1), Position::new(2, 11)),
                    LexerElem::new(Elem::Text("v"), Position::new(2, 12)),
                    LexerElem::new(Elem::MacroParam, Position::new(2, 13)),
                    LexerElem::new(Elem::Number(2), Position::new(2, 14)),
                    LexerElem::new(Elem::BeginGroup, Position::new(2, 15)),
                    LexerElem::new(Elem::Text("bar"), Position::new(2, 16)),
                    LexerElem::new(Elem::EndGroup, Position::new(2, 19)),
                    LexerElem::new(Elem::EndOfFile, Position::new(2, 20))];

    for d in data {
        assert_eq!(lexer.next(), d);
    }

    assert_eq!(lexer.next(),
               LexerElem::new(Elem::EndOfFile, Position::new(2, 20)));
}

#[test]
fn test_peek_next() {
    let input = "\\foo a string\\bar
{a string enclosed by a group}";

    let lexer = Lexer::new(input);

    let data = vec![LexerElem::new(Elem::Command("foo"), Position::new(0, 0)),
                    LexerElem::new(Elem::Text(" a string"), Position::new(0, 4)),
                    LexerElem::new(Elem::Command("bar"), Position::new(0, 13)),
                    LexerElem::new(Elem::LineBreak, Position::new(0, 17)),
                    LexerElem::new(Elem::BeginGroup, Position::new(1, 0)),
                    LexerElem::new(Elem::Text("a string enclosed by a group"),
                                   Position::new(1, 1)),
                    LexerElem::new(Elem::EndGroup, Position::new(1, 29)),
                    LexerElem::new(Elem::EndOfFile, Position::new(1, 30))];

    for d in &data {
        assert_eq!(lexer.peek_next(), *d.elem());
    }

    lexer.reset_peek();

    for d in &data {
        assert_eq!(lexer.peek_next(), *d.elem());
    }

    for d in data {
        assert_eq!(lexer.next(), d);
    }
}
