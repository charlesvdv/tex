use std::cell::Cell;
use lexer::Position;

#[derive(Default, Debug)]
pub struct ZeroCopyStreamer<'a> {
    input: &'a str,
    pos: Cell<Position>,
    offset: Cell<usize>,
    // Offset used to move from character to character when we call
    // peek_char.
    peek_offset: Cell<usize>,
}

impl<'a> ZeroCopyStreamer<'a> {
    pub fn new(input: &'a str) -> Self {
        ZeroCopyStreamer {
            input,
            ..Default::default()
        }
    }

    pub fn read_char(&self) -> Option<char> {
        match self.input[self.offset.get()..].chars().next() {
            Some(v) => {
                self.update_pos(v.len_utf8());
                Some(v)
            }
            None => None,
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        match self.input[self.offset.get() + self.peek_offset.get()..]
                  .chars()
                  .next() {
            Some(v) => {
                self.peek_offset.set(self.peek_offset.get() + v.len_utf8());
                Some(v)
            }
            None => None,
        }
    }

    pub fn take_while<P>(&self, predicate: P) -> &str
        where P: Fn(char) -> bool
    {
        let mut bytes_len = 0;
        loop {
            match self.peek_char() {
                Some(v) if predicate(v) => {
                    bytes_len += v.len_utf8();
                }
                _ => break,
            }
        }

        let old_offset = self.offset.get();
        self.update_pos(bytes_len + 1);
        &self.input[old_offset..self.offset.get()]
    }

    pub fn get_pos(&self) -> Position {
        self.pos.get().clone()
    }

    pub fn set_newline(&self) {
        let mut pos = self.pos.get();
        pos.line += 1;
        pos.column = 0;
        self.pos.set(pos);
    }

    // Update both the offset and the `pos` used by the
    // lexer.
    fn update_pos(&self, bytes_len: usize) {
        // Update the position.
        let mut pos = self.pos.get();
        pos.column += bytes_len;
        self.pos.set(pos);
        // Update the offset.
        self.offset.set(self.offset.get() + bytes_len);
        // As we consumed characters, reset the peek offset.
        self.peek_offset.set(0);
    }
}
