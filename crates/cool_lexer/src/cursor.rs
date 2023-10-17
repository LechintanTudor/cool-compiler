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
    #[must_use]
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
    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
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
