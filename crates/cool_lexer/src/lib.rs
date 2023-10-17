mod consts;
mod cursor;
mod punct;
mod symbol;
mod token;

pub use self::consts::*;
pub use self::cursor::*;
pub use self::punct::*;
pub use self::symbol::*;
pub use self::token::*;

pub mod tk {
    pub use crate::punct_tk::*;
    pub use crate::sym_tk::*;
}

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    buffer: String,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor::new(source),
            buffer: String::new(),
        }
    }

    #[must_use]
    pub fn next_token(&mut self) -> Token {
        todo!()
    }

    #[inline]
    #[must_use]
    fn is_ident_start(c: char) -> bool {
        unicode_ident::is_xid_start(c) || c == '_'
    }

    #[inline]
    #[must_use]
    fn is_ident_continue(c: char) -> bool {
        unicode_ident::is_xid_continue(c) || c == '_'
    }

    #[inline]
    #[must_use]
    fn is_whitespace(c: char) -> bool {
        matches!(
            c,
            // Usual ASCII suspects
            '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
        )
    }
}
