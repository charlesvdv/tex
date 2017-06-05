use super::catcodes::Catcodes;
use super::*;

#[test]
fn test_basic_lexer() {
    let catcodes = Catcodes::default();

    let input = "foo bar \\{
\\bar \\foo{bar}";

    let mut lexer = Lexer::new(input);

    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::Text("foo bar ".into()), Position::new(0, 0))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::EscapedChar('{'), Position::new(0, 8))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::LineBreak, Position::new(0, 10))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::Control("bar".into()), Position::new(1, 0))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::Space, Position::new(1, 4))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::Control("foo".into()), Position::new(1, 5))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::BeginGroup, Position::new(1, 9))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::Text("bar".into()), Position::new(1, 10))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::EndGroup, Position::new(1, 13))
    );
    assert_eq!(
        lexer.next(&catcodes),
        LexerElem::new(Elem::EndOfFile, Position::new(1, 14))
    );
}
