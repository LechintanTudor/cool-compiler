use std::str::Chars;

pub const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    offset: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str, offset: u32) -> Self {
        Self {
            chars: source.chars(),
            offset,
        }
    }

    pub fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(char) => {
                self.offset += char.len_utf8() as u32;
                char
            }
            None => EOF_CHAR,
        }
    }

    pub fn bump_with_offset(&mut self) -> (u32, char) {
        match self.chars.next() {
            Some(char) => {
                let offset = self.offset;
                self.offset += char.len_utf8() as u32;
                (offset, char)
            }
            None => (self.offset, EOF_CHAR),
        }
    }

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn consume_if<P>(&mut self, predicate: P) -> bool
    where
        P: FnOnce(char) -> bool,
    {
        if !predicate(self.peek()) || self.is_eof() {
            return false;
        }

        self.bump();
        true
    }

    pub fn consume_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(char) -> bool,
    {
        while predicate(self.peek()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn consume_for<P, F>(&mut self, predicate: P, mut function: F)
    where
        P: Fn(char) -> bool,
        F: FnMut(char),
    {
        while predicate(self.peek()) && !self.is_eof() {
            let char = self.bump();
            function(char);
        }
    }
}
