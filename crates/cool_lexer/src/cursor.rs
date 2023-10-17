use std::str::Chars;

pub const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    offset: u32,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            offset: 0,
        }
    }

    #[inline]
    pub fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(char) => {
                self.offset += char.len_utf8() as u32;
                char
            }
            None => EOF_CHAR,
        }
    }

    #[inline]
    #[must_use]
    pub fn bump_with_offset(&mut self) -> (char, u32) {
        match self.chars.next() {
            Some(char) => {
                let offset = self.offset;
                self.offset += char.len_utf8() as u32;
                (char, offset)
            }
            None => (EOF_CHAR, self.offset),
        }
    }

    #[inline]
    #[must_use]
    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn consume_if<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        if !f(self.peek()) || self.is_eof() {
            return false;
        }

        self.bump();
        true
    }

    pub fn consume_while<F>(&mut self, mut f: F)
    where
        F: FnMut(char) -> bool,
    {
        while f(self.peek()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn push_if<F>(&mut self, buffer: &mut String, mut f: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        if !f(self.peek()) || self.is_eof() {
            return false;
        }

        buffer.push(self.bump());
        true
    }

    pub fn push_while<F>(&mut self, buffer: &mut String, mut f: F)
    where
        F: FnMut(char) -> bool,
    {
        while f(self.peek()) && !self.is_eof() {
            buffer.push(self.bump());
        }
    }

    #[inline]
    #[must_use]
    pub fn offset(&self) -> u32 {
        self.offset
    }

    #[inline]
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
}
