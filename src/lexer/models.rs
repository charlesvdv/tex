#[derive(Debug, PartialEq, Eq)]
pub struct LexerElem {
    pub elem: Elem,
    pub pos: Position,
}

impl LexerElem {
    pub fn new(elem: Elem, pos: Position) -> Self {
        LexerElem { elem, pos }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    Text(String),
    Control(String),
    Comment(String),
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
    pub line: i32,
    pub column: i32,
}

impl Position {
    pub fn new(line: i32, column: i32) -> Self {
        Position { line, column }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn char(&mut self) {
        self.column += 1;
    }

    pub fn chars(&mut self, size: usize) {
        self.column += size as i32;
    }
}
