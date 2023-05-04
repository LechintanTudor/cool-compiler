use std::str::Chars;

pub const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    offset: u32,
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            offset: 0,
        }
    }
}

impl Cursor<'_> {
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

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
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
        if !predicate(self.first()) || self.is_eof() {
            return false;
        }

        self.bump();
        true
    }

    pub fn consume_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(char) -> bool,
    {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
