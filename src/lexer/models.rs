#[derive(Debug, PartialEq, Eq)]
pub struct LexerElem<'a> {
    pub elem: Elem<'a>,
    pub pos: Position,
}

impl<'a> LexerElem<'a> {
    pub fn new(elem: Elem<'a>, pos: Position) -> Self {
        LexerElem { elem, pos }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Elem<'a> {
    Text(&'a str),
    Control(&'a str),
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn advance(&mut self, bytes_len: usize) {
        self.column += bytes_len;
    }
}
