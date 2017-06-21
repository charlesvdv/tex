use std::collections::VecDeque;
use std::cell::{Cell, RefCell};
use lexer::{LexTokenIterator, Catcodes, Token, LexerToken, ZeroCopyStreamer};

#[derive(Debug)]
pub struct Lexer<'a> {
    streamer: ZeroCopyStreamer<'a>,
    catcodes: Catcodes,
    // Used by `peek_next` function.
    peeked: RefCell<VecDeque<LexerToken<'a>>>,
    peek_offset: Cell<usize>,
}

impl<'a> LexTokenIterator<'a> for Lexer<'a> {
    fn next(&self) -> Option<LexerToken<'a>> {
        self.reset_peek();

        // Check if we have peeked element before tokenizing the input.
        if self.peeked.borrow().is_empty() {
            Some(self.tokenize_element())
        } else {
            Some(self.peeked.borrow_mut().pop_front().unwrap())
        }
    }

    fn peek_next(&self) -> Option<Token<'a>> {
        // Check if we need to tokenize from a new input or we can use
        // tokenized data.
        if self.peek_offset.get() == self.peeked.borrow().len() {
            self.peek_offset.set(self.peek_offset.get() + 1);
            // Get the newest token.
            let token = self.tokenize_element();
            self.peeked.borrow_mut().push_back(token);

            let peeked = self.peeked.borrow();
            Some(peeked.back().unwrap().elem().clone())
        } else if self.peek_offset.get() < self.peeked.borrow().len() {
            self.peek_offset.set(self.peek_offset.get() + 1);

            let peeked = self.peeked.borrow();
            Some(
                peeked
                    .get(self.peek_offset.get() - 1)
                    .unwrap()
                    .elem()
                    .clone(),
            )
        } else {
            unreachable!();
        }
    }

    fn reset_peek(&self) {
        self.peek_offset.set(0);
    }

    fn set_catcode(&mut self, catcodes: Catcodes) {
        self.catcodes = catcodes;
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            streamer: ZeroCopyStreamer::new(input),
            catcodes: Catcodes::default(),
            peeked: RefCell::new(VecDeque::new()),
            peek_offset: Cell::new(0),
        }
    }

    fn tokenize_element(&self) -> LexerToken<'a> {
        let pos = self.streamer.get_pos();

        let elem = match self.streamer.peek_char() {
            Some(v) if self.catcodes.is_begin_group(v) => {
                self.streamer.read_char();
                Token::BeginGroup
            }
            Some(v) if self.catcodes.is_end_group(v) => {
                self.streamer.read_char();
                Token::EndGroup
            }
            Some(v) if self.catcodes.is_math_shift(v) => {
                self.streamer.read_char();
                Token::MathShift
            }
            Some(v) if self.catcodes.is_alignement_tab(v) => {
                self.streamer.read_char();
                Token::AlignementTab
            }
            Some(v) if self.catcodes.is_subscript(v) => {
                self.streamer.read_char();
                Token::SubScript
            }
            Some(v) if self.catcodes.is_superscript(v) => {
                self.streamer.read_char();
                Token::SuperScript
            }
            Some(v) if self.catcodes.is_active(v) => {
                self.streamer.read_char();
                Token::Active
            }
            Some(v) if self.catcodes.is_macro_param(v) => {
                self.streamer.read_char();
                Token::MacroParam
            }

            Some(v) if self.catcodes.is_end_of_line(v) => {
                self.streamer.read_char();
                self.streamer.set_newline();
                Token::LineBreak
            }
            Some(v) if self.catcodes.is_control_sequence(v) => {
                // Consume the `\` char.
                self.streamer.read_char();
                self.elem_control_sequence()
            }
            Some(v) if self.catcodes.is_space(v) => {
                match self.streamer.peek_char() {
                    Some(v) if v.is_alphabetic() => Token::Text(self.text()),
                    _ => {
                        self.streamer.read_char();
                        Token::Space
                    }
                }
            }
            Some(v) if self.catcodes.is_letter(v) => Token::Text(self.text()),
            Some(v) if v.is_numeric() => Token::Number(self.number()),
            Some(v) if self.catcodes.is_comment(v) => {
                Token::Comment(
                    self.streamer
                        .take_while(|x| !self.catcodes.is_end_of_line(x)),
                )
            }
            Some(v) => {
                self.streamer.read_char();
                Token::SpecialChar(v)
            }

            None => Token::EndOfFile,
        };
        LexerToken::new(elem, pos)
    }

    // Match everything that begin by the control sequence character
    // (in most case: `\`).
    fn elem_control_sequence(&self) -> Token<'a> {
        match self.streamer.peek_char() {
            Some(v) if self.catcodes.is_escaped_char(v) => {
                self.streamer.read_char();
                Token::EscapedChar(v)
            }
            Some(v) if self.catcodes.is_other_character(v) => {
                self.streamer.read_char();
                Token::SpecialChar(v)
            }
            Some(v) if self.catcodes.is_letter(v) => {
                Token::Command(self.streamer.take_while(|x| self.catcodes.is_letter(x)))
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
        self.streamer.take_while(|x| {
            !self.catcodes.is_control_sequence(x) && !self.catcodes.is_end_of_line(x) &&
                !self.catcodes.is_escaped_char(x)
        })
    }
}
