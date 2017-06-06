use super::catcodes::Catcodes;
use super::*;

#[test]
fn test_basic_lexer() {
    let catcodes = Catcodes::default();

    let input = "foo bar \\{
\\bar \\foo{bar}
\\def \\foo #1v#2{bar}";

    let mut lexer = Lexer::new(input);

    let data = vec![
        LexerElem::new(Elem::Text("foo bar ".into()), Position::new(0, 0)),
        LexerElem::new(Elem::EscapedChar('{'), Position::new(0, 8)),
        LexerElem::new(Elem::LineBreak, Position::new(0, 10)),
        LexerElem::new(Elem::Control("bar".into()), Position::new(1, 0)),
        LexerElem::new(Elem::Space, Position::new(1, 4)),
        LexerElem::new(Elem::Control("foo".into()), Position::new(1, 5)),
        LexerElem::new(Elem::BeginGroup, Position::new(1, 9)),
        LexerElem::new(Elem::Text("bar".into()), Position::new(1, 10)),
        LexerElem::new(Elem::EndGroup, Position::new(1, 13)),
        LexerElem::new(Elem::LineBreak, Position::new(1, 14)),
        LexerElem::new(Elem::Control("def".into()), Position::new(2, 0)),
        LexerElem::new(Elem::Space, Position::new(2, 4)),
        LexerElem::new(Elem::Control("foo".into()), Position::new(2, 5)),
        LexerElem::new(Elem::Space, Position::new(2, 9)),
        LexerElem::new(Elem::MacroParam, Position::new(2, 10)),
        LexerElem::new(Elem::Number(1), Position::new(2, 11)),
        LexerElem::new(Elem::Text("v".into()), Position::new(2, 12)),
        LexerElem::new(Elem::MacroParam, Position::new(2, 13)),
        LexerElem::new(Elem::Number(2), Position::new(2, 14)),
        LexerElem::new(Elem::BeginGroup, Position::new(2, 15)),
        LexerElem::new(Elem::Text("bar".into()), Position::new(2, 16)),
        LexerElem::new(Elem::EndGroup, Position::new(2, 19)),
        LexerElem::new(Elem::EndOfFile, Position::new(2, 20)),
    ];

    for d in data {
        assert_eq!(lexer.next(&catcodes), d);
    }
}
