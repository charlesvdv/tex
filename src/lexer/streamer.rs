use lexer::Position;

#[derive(Default, Debug)]
pub struct ZeroCopyStreamer<'a> {
    input: &'a str,
    pos: Position,
    offset: usize,
    // Offset used to move from character to character when we call
    // peek_char.
    peek_offset: usize,
}

impl<'a> ZeroCopyStreamer<'a> {
    pub fn new(input: &'a str) -> Self {
        ZeroCopyStreamer {
            input,
            ..Default::default()
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        match self.input[self.offset..].chars().next() {
            Some(v) => {
                self.update_pos(v.len_utf8());
                Some(v)
            }
            None => None,
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        match self.input[self.offset + self.peek_offset..].chars().next() {
            Some(v) => {
                self.peek_offset += v.len_utf8();
                Some(v)
            }
            None => None,
        }
    }

    pub fn take_while<P>(&mut self, predicate: P) -> &str
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

        let old_offset = self.offset;
        self.update_pos(bytes_len + 1);
        &self.input[old_offset..self.offset]
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }

    pub fn set_newline(&mut self) {
        self.pos.new_line();
    }

    // Update both the offset and the `pos` used by the
    // lexer.
    fn update_pos(&mut self, bytes_len: usize) {
        self.pos.advance(bytes_len);
        self.offset += bytes_len;
        // As we consumed characters, reset the peek offset.
        self.peek_offset = 0;
    }

}
