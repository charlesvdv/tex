use std::collections::VecDeque;
use lexer::{Catcodes, Elem, LexerElem, ZeroCopyStreamer};

#[derive(Debug)]
pub struct Lexer<'a> {
    streamer: ZeroCopyStreamer<'a>,
    catcodes: Catcodes,
    // Used by `peek_next` function.
    peeked: VecDeque<LexerElem<'a>>,
    peek_offset: usize,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            streamer: ZeroCopyStreamer::new(input),
            catcodes: Catcodes::default(),
            peeked: VecDeque::new(),
            peek_offset: 0,
        }
    }

    pub fn next(&mut self) -> LexerElem<'a> {
        self.reset_peek();

        if self.peeked.is_empty() {
            self.tokenize_element()
        } else {
            self.peeked.pop_front().unwrap()
        }
    }

    pub fn peek_next(&mut self) -> &LexerElem<'a> {
        if self.peek_offset == self.peeked.len() {
            self.peek_offset += 1;
            // Get the newest token.
            let token = self.tokenize_element();
            self.peeked.push_back(token);
            self.peeked.back().unwrap()
        } else if self.peek_offset < self.peeked.len() {
            self.peek_offset += 1;
            self.peeked.get(self.peek_offset - 1).unwrap()
        } else {
            unreachable!();
        }
    }

    pub fn reset_peek(&mut self) {
        self.peek_offset = 0;
    }

    pub fn set_catcode(&mut self, code: usize, value: char) {
        self.catcodes.set_catcode(code, value);
    }

    fn tokenize_element(&self) -> LexerElem<'a> {
        let pos = self.streamer.get_pos();

        let elem = match self.streamer.peek_char() {
            Some(v) if self.catcodes.is_begin_group(v) => {
                self.streamer.read_char();
                Elem::BeginGroup
            }
            Some(v) if self.catcodes.is_end_group(v) => {
                self.streamer.read_char();
                Elem::EndGroup
            }
            Some(v) if self.catcodes.is_math_shift(v) => {
                self.streamer.read_char();
                Elem::MathShift
            }
            Some(v) if self.catcodes.is_alignement_tab(v) => {
                self.streamer.read_char();
                Elem::AlignementTab
            }
            Some(v) if self.catcodes.is_subscript(v) => {
                self.streamer.read_char();
                Elem::SubScript
            }
            Some(v) if self.catcodes.is_superscript(v) => {
                self.streamer.read_char();
                Elem::SuperScript
            }
            Some(v) if self.catcodes.is_active(v) => {
                self.streamer.read_char();
                Elem::Active
            }
            Some(v) if self.catcodes.is_macro_param(v) => {
                self.streamer.read_char();
                Elem::MacroParam
            }

            Some(v) if self.catcodes.is_end_of_line(v) => {
                self.streamer.read_char();
                self.streamer.set_newline();
                Elem::LineBreak
            }
            Some(v) if self.catcodes.is_control_sequence(v) => {
                // Consume the `\` char.
                self.streamer.read_char();
                self.elem_control_sequence()
            }
            Some(v) if self.catcodes.is_space(v) => {
                match self.streamer.peek_char() {
                    Some(v) if v.is_alphabetic() => Elem::Text(self.text()),
                    _ => {
                        self.streamer.read_char();
                        Elem::Space
                    }
                }
            }
            Some(v) if self.catcodes.is_letter(v) => Elem::Text(self.text()),
            Some(v) if v.is_numeric() => Elem::Number(self.number()),
            Some(v) if self.catcodes.is_comment(v) => {
                Elem::Comment(self.streamer
                                  .take_while(|x| !self.catcodes.is_end_of_line(x)))
            }
            Some(v) => {
                self.streamer.read_char();
                Elem::SpecialChar(v)
            }

            None => Elem::EndOfFile,
        };
        LexerElem::new(elem, pos)
    }

    // Match everything that begin by the control sequence character
    // (in most case: `\`).
    fn elem_control_sequence(&self) -> Elem<'a> {
        match self.streamer.peek_char() {
            Some(v) if self.catcodes.is_escaped_char(v) => {
                self.streamer.read_char();
                Elem::EscapedChar(v)
            }
            Some(v) if self.catcodes.is_other_character(v) => {
                self.streamer.read_char();
                Elem::SpecialChar(v)
            }
            Some(v) if self.catcodes.is_letter(v) => {
                Elem::Command(self.streamer
                                  .take_while(|x| {
                                                  self.catcodes.is_letter(x) ||
                                                  self.catcodes.is_other_character(x)
                                              }))
            }
            _ => unreachable!(),
        }
    }

    // TODO: handle negative number?
    fn number(&self) -> i32 {
        self.streamer
            .take_while(|x| x.is_numeric())
            .parse()
            .unwrap()
    }

    fn text(&self) -> &'a str {
        self.streamer
            .take_while(|x| {
                            !self.catcodes.is_control_sequence(x) &&
                            !self.catcodes.is_end_of_line(x) &&
                            !self.catcodes.is_escaped_char(x)
                        })
    }
}
