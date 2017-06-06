mod models;
mod catcodes;
#[cfg(test)]
mod tests;

pub use self::models::*;
use self::catcodes::Catcodes;


#[derive(Debug, Default)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: Position,
    offset: usize,
    peek_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            ..Default::default()
        }
    }

    pub fn next(&mut self, catcodes: &Catcodes) -> LexerElem {
        let pos = self.pos.clone();

        let elem = match self.peek_char() {
            Some(v) if catcodes.is_begin_group(v) => {
                self.read_char();
                Elem::BeginGroup
            },
            Some(v) if catcodes.is_end_group(v) => {
                self.read_char();
                Elem::EndGroup
            },
            Some(v) if catcodes.is_math_shift(v) => {
                self.read_char();
                Elem::MathShift
            },
            Some(v) if catcodes.is_alignement_tab(v) => {
                self.read_char();
                Elem::AlignementTab
            },
            Some(v) if catcodes.is_subscript(v) => {
                self.read_char();
                Elem::SubScript
            },
            Some(v) if catcodes.is_superscript(v) => {
                self.read_char();
                Elem::SuperScript
            },
            Some(v) if catcodes.is_active(v) => {
                self.read_char();
                Elem::Active
            },
            Some(v) if catcodes.is_macro_param(v) => {
                self.read_char();
                Elem::MacroParam
            },

            Some(v) if catcodes.is_end_of_line(v) => {
                self.read_char();
                self.pos.new_line();
                Elem::LineBreak
            },
            Some(v) if catcodes.is_control_sequence(v) => {
                // Consume the `\` char.
                self.read_char();
                self.elem_control_sequence(catcodes)
            },
            Some(v) if catcodes.is_space(v) => {
                match self.peek_char() {
                    Some(v) if v.is_alphabetic() => {
                        Elem::Text(self.text(catcodes))
                    },
                    _ => {
                        self.read_char();
                        Elem::Space
                    },
                }
            },
            Some(v) if catcodes.is_letter(v) => {
                Elem::Text(self.text(catcodes))
            },
            Some(v) if v.is_numeric() => {
                Elem::Number(self.number())
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
            Some(v) if catcodes.is_escaped_char(v) => {
                self.read_char();
                Elem::EscapedChar(v)
            },
            Some(v) if catcodes.is_other_character(v) => {
                self.read_char();
                Elem::SpecialChar(v)
            },
            Some(v) if catcodes.is_letter(v) => {
                Elem::Control(self.take_while(|x| {
                    catcodes.is_letter(x) ||
                    catcodes.is_other_character(x)
                }))
            },
            _ => unreachable!()
        }
    }

    // TODO: handle negative number?
    fn number(&mut self) -> i32 {
        self.take_while(|x| x.is_numeric()).parse().unwrap()
    }

    fn text(&mut self, catcodes: &Catcodes) -> &str {
        self.take_while(|x| {
            !catcodes.is_control_sequence(x) &&
            !catcodes.is_end_of_line(x) &&
            !catcodes.is_escaped_char(x)
        })
    }

    fn read_char(&mut self) -> Option<char> {
        match self.input[self.offset..].chars().next() {
            Some(v) => {
                self.advance_pos(v.len_utf8());
                Some(v)
            },
            None => None,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        match self.input[self.offset+self.peek_offset..].chars().next() {
            Some(v) => {
                self.peek_offset += v.len_utf8();
                Some(v)
            },
            None => None,
        }
    }

    fn take_while<P>(&mut self, predicate: P) -> &str where P: Fn(char) -> bool {
        let mut bytes_len = 0;
        loop {
            match self.peek_char() {
                Some(v) if predicate(v) => {
                    bytes_len += v.len_utf8();
                },
                _ => break,
            }
        }
        let old_offset = self.offset;
        self.advance_pos(bytes_len + 1);
        &self.input[old_offset..old_offset+bytes_len+1]
    }

    fn advance_pos(&mut self, size: usize) {
        self.offset += size;
        self.peek_offset = 0;

        self.pos.advance(size);
    }
}
