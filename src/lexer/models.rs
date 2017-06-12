#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerElem<'a> {
    elem: Elem<'a>,
    pos: Position,
}

impl<'a> LexerElem<'a> {
    pub fn new(elem: Elem<'a>, pos: Position) -> Self {
        LexerElem { elem, pos }
    }

    pub fn elem(&self) -> &Elem<'a> {
        &self.elem
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Elem<'a> {
    Text(&'a str),
    Command(&'a str),
    Comment(&'a str),
    Number(i32),
    EscapedChar(char),
    SpecialChar(char),
    LineBreak,
    EndOfFile,
    Space,
    BeginGroup,
    EndGroup,
    MathShift,
    AlignementTab,
    SuperScript,
    SubScript,
    Active,
    MacroParam,

    Char(char),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}
