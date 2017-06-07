use lexer::{Catcodes, Elem, LexerElem, ZeroCopyStreamer};

#[derive(Debug, Default)]
pub struct Lexer<'a> {
    streamer: ZeroCopyStreamer<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            streamer: ZeroCopyStreamer::new(input),
            ..Default::default()
        }
    }

    pub fn next(&mut self, catcodes: &Catcodes) -> LexerElem {
        let pos = self.streamer.get_pos();

        let elem = match self.streamer.peek_char() {
            Some(v) if catcodes.is_begin_group(v) => {
                self.streamer.read_char();
                Elem::BeginGroup
            }
            Some(v) if catcodes.is_end_group(v) => {
                self.streamer.read_char();
                Elem::EndGroup
            }
            Some(v) if catcodes.is_math_shift(v) => {
                self.streamer.read_char();
                Elem::MathShift
            }
            Some(v) if catcodes.is_alignement_tab(v) => {
                self.streamer.read_char();
                Elem::AlignementTab
            }
            Some(v) if catcodes.is_subscript(v) => {
                self.streamer.read_char();
                Elem::SubScript
            }
            Some(v) if catcodes.is_superscript(v) => {
                self.streamer.read_char();
                Elem::SuperScript
            }
            Some(v) if catcodes.is_active(v) => {
                self.streamer.read_char();
                Elem::Active
            }
            Some(v) if catcodes.is_macro_param(v) => {
                self.streamer.read_char();
                Elem::MacroParam
            }

            Some(v) if catcodes.is_end_of_line(v) => {
                self.streamer.read_char();
                self.streamer.set_newline();
                Elem::LineBreak
            }
            Some(v) if catcodes.is_control_sequence(v) => {
                // Consume the `\` char.
                self.streamer.read_char();
                self.elem_control_sequence(catcodes)
            }
            Some(v) if catcodes.is_space(v) => {
                match self.streamer.peek_char() {
                    Some(v) if v.is_alphabetic() => Elem::Text(self.text(catcodes)),
                    _ => {
                        self.streamer.read_char();
                        Elem::Space
                    }
                }
            }
            Some(v) if catcodes.is_letter(v) => Elem::Text(self.text(catcodes)),
            Some(v) if v.is_numeric() => Elem::Number(self.number()),
            Some(v) if catcodes.is_comment(v) => {
                Elem::Comment(self.streamer.take_while(|x| !catcodes.is_end_of_line(x)))
            }
            Some(v) => {
                self.streamer.read_char();
                Elem::SpecialChar(v)
            },

            None => Elem::EndOfFile,
        };
        LexerElem::new(elem, pos)
    }

    fn elem_control_sequence(&mut self, catcodes: &Catcodes) -> Elem {
        match self.streamer.peek_char() {
            Some(v) if catcodes.is_escaped_char(v) => {
                self.streamer.read_char();
                Elem::EscapedChar(v)
            }
            Some(v) if catcodes.is_other_character(v) => {
                self.streamer.read_char();
                Elem::SpecialChar(v)
            }
            Some(v) if catcodes.is_letter(v) => {
                Elem::Control(self.streamer.take_while(|x| {
                                                  catcodes.is_letter(x) ||
                                                  catcodes.is_other_character(x)
                                              }))
            }
            _ => unreachable!(),
        }
    }

    // TODO: handle negative number?
    fn number(&mut self) -> i32 {
        self.streamer.take_while(|x| x.is_numeric()).parse().unwrap()
    }

    fn text(&mut self, catcodes: &Catcodes) -> &str {
        self.streamer.take_while(|x| {
                            !catcodes.is_control_sequence(x) && !catcodes.is_end_of_line(x) &&
                            !catcodes.is_escaped_char(x)
                        })
    }
}
