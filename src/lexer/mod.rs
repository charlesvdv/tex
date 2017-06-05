use std::str::Chars;
use std::iter::Peekable;

mod models;
mod catcodes;
#[cfg(test)]
mod tests;

pub use self::models::*;
use self::catcodes::Catcodes;


#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(tex: &'a str) -> Self {
        Lexer {
            input: tex.chars().peekable(),
            pos: Position::default(),
        }
    }

    pub fn next(&mut self, catcodes: &Catcodes) -> LexerElem {
        let pos = self.pos.clone();

        let elem = match self.read_char() {
            // Catcodes requiring basic handling.
            Some(v) if catcodes.is_begin_group(v) => Elem::BeginGroup,
            Some(v) if catcodes.is_end_group(v) => Elem::EndGroup,
            Some(v) if catcodes.is_math_shift(v) => Elem::MathShift,
            Some(v) if catcodes.is_alignement_tab(v) => Elem::AlignementTab,
            Some(v) if catcodes.is_subscript(v) => Elem::SubScript,
            Some(v) if catcodes.is_superscript(v) => Elem::SuperScript,
            Some(v) if catcodes.is_active(v) => Elem::Active,
            Some(v) if catcodes.is_macro_param(v) => Elem::MacroParam,

            Some(v) if catcodes.is_end_of_line(v) => {
                self.pos.new_line();
                Elem::LineBreak
            },
            Some(v) if catcodes.is_control_sequence(v) => {
                self.elem_control_sequence(catcodes)
            },
            Some(v) if catcodes.is_space(v) => {
                match self.peek_char() {
                    Some(&v) if v.is_alphabetic() => {
                        Elem::Text(self.text(' ', catcodes))
                    },
                    _ => Elem::Space,
                }
            },
            Some(v) if catcodes.is_letter(v) => {
                Elem::Text(self.text(v, catcodes))
            },
            Some(v) if v.is_numeric() => {
                Elem::Number(self.number(v))
            },
            Some(v) if catcodes.is_comment(v) => {
                Elem::Comment(self.take_while(|x| {
                    !catcodes.is_end_of_line(x)
                }))
            },
            Some(v) => Elem::SpecialChar(v),

            None => Elem::EndOfFile,
        };
        LexerElem::new(elem, pos)
    }

    fn elem_control_sequence(&mut self, catcodes: &Catcodes) -> Elem {
        match self.peek_char() {
            Some(&v) if catcodes.is_escaped_char(v) => {
                self.read_char();
                Elem::EscapedChar(v)
            },
            Some(&v) if catcodes.is_other_character(v) => {
                self.read_char();
                Elem::SpecialChar(v)
            },
            Some(&v) if catcodes.is_letter(v) => {
                Elem::Control(self.take_while(|x| {
                    catcodes.is_letter(x) ||
                    catcodes.is_other_character(x)
                }))
            },
            _ => unreachable!()
        }
    }

    // TODO: handle negative number?
    fn number(&mut self, first_num: char) -> i32 {
        let mut num = self.take_while(|x| x.is_numeric());
        num.insert(0, first_num);

        num.parse().unwrap()
    }

    fn text(&mut self, first_ch: char, catcodes: &Catcodes) -> String {
        let mut txt = self.take_while(|x| {
            !catcodes.is_control_sequence(x) &&
            !catcodes.is_end_of_line(x) &&
            !catcodes.is_escaped_char(x)
        });
        txt.insert(0, first_ch);

        txt
    }

    fn read_char(&mut self) -> Option<char> {
        self.pos.char();
        self.input.next()
    }

    fn take_while<P>(&mut self, predicate: P) -> String where P: Fn(char) -> bool {
        let mut data = String::new();
        loop {
            match self.peek_char() {
                Some(&v) => {
                    if predicate(v) {
                        data.push(self.read_char().unwrap());
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }
        // take_while consume the last token checked. Not very usefull in this case...
        // let data: String = self.input.by_ref().take_while(predicate).collect();

        data
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }
}
