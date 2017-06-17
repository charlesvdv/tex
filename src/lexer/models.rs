#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerToken<'a> {
    elem: Token<'a>,
    pos: Position,
}

impl<'a> LexerToken<'a> {
    pub fn new(elem: Token<'a>, pos: Position) -> Self {
        LexerToken { elem, pos }
    }

    pub fn elem(&self) -> &Token<'a> {
        &self.elem
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn consume(self) -> (Token<'a>, Position) {
        (self.elem, self.pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
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
