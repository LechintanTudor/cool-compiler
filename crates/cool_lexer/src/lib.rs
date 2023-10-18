mod consts;
mod cursor;
mod literal;
mod punct;
mod symbol;
mod token;
mod token_stream;

pub use self::consts::*;
pub use self::cursor::*;
pub use self::literal::*;
pub use self::punct::*;
pub use self::symbol::*;
pub use self::token::*;
pub use self::token_stream::*;

use cool_span::Span;

pub mod tk {
    pub use crate::punct_tk::*;
    pub use crate::sym_tk::*;
}

#[derive(Clone, Debug)]
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
        let (c, offset) = self.cursor.bump_with_offset();

        let kind = if chars::is_ident_start(c) {
            self.buffer.push(c);
            self.ident_or_keyword()
        } else if c == '/' && self.cursor.consume_if(|c| c == '/') {
            self.comment()
        } else if c == '\'' {
            self.char_or_str_literal(LiteralKind::Char)
        } else if c == '"' {
            self.char_or_str_literal(LiteralKind::Str)
        } else if let Ok(punct) = Punct::try_from(c) {
            self.punct(punct)
        } else if chars::is_decimal_digit(c) {
            self.buffer.push(c);

            match self.cursor.peek() {
                'b' => {
                    self.buffer.push(self.cursor.bump());
                    self.int_literal(true, chars::is_binary_digit)
                }
                'o' => {
                    self.buffer.push(self.cursor.bump());
                    self.int_literal(true, chars::is_octal_digit)
                }
                'x' => {
                    self.buffer.push(self.cursor.bump());
                    self.int_literal(true, chars::is_hexadecimal_digit)
                }
                _ => self.int_literal(false, chars::is_decimal_digit),
            }
        } else if chars::is_whitespace(c) {
            self.whitespace()
        } else if c == EOF_CHAR {
            TokenKind::Eof
        } else {
            TokenKind::Unknown
        };

        Token {
            span: Span::from_to(offset, self.cursor.offset()),
            kind,
        }
    }

    fn ident_or_keyword(&mut self) -> TokenKind {
        while chars::is_ident_continue(self.cursor.peek()) {
            self.buffer.push(self.cursor.bump());
        }

        let symbol = Symbol::insert(&self.buffer);
        self.buffer.clear();

        if symbol.is_keyword() {
            TokenKind::Keyword(symbol)
        } else if symbol.is_bool_literal() {
            Literal {
                kind: LiteralKind::Bool,
                value: symbol,
            }
            .into()
        } else {
            TokenKind::Ident(symbol)
        }
    }

    #[inline]
    fn comment(&mut self) -> TokenKind {
        self.cursor.consume_while(|c| c != '\n');
        self.cursor.consume_if(|c| c == '\n');
        TokenKind::Comment
    }

    fn punct(&mut self, start: Punct) -> TokenKind {
        let mut punct = start;

        while let Ok(next_punct) = Punct::try_from(self.cursor.peek()) {
            let Ok(joined_punct) = punct.join(next_punct) else {
                break;
            };

            self.cursor.bump();
            punct = joined_punct;
        }

        punct.into()
    }

    fn int_literal<F>(&mut self, requires_digit: bool, mut is_digit: F) -> TokenKind
    where
        F: FnMut(char) -> bool,
    {
        let mut has_digit = false;
        self.cursor.push_while(&mut self.buffer, |c| {
            if is_digit(c) {
                has_digit = true;
                return true;
            }

            c == '_'
        });

        if requires_digit && !has_digit {
            return TokenKind::Unknown;
        }

        if self
            .cursor
            .push_if(&mut self.buffer, chars::is_suffix_start)
        {
            self.cursor
                .push_while(&mut self.buffer, chars::is_ident_continue)
        }

        let symbol = Symbol::insert(&self.buffer);
        self.buffer.clear();

        Literal {
            kind: LiteralKind::Int,
            value: symbol,
        }
        .into()
    }

    fn char_or_str_literal(&mut self, kind: LiteralKind) -> TokenKind {
        let quote = match kind {
            LiteralKind::Char => '\'',
            LiteralKind::Str => '"',
            _ => panic!("Invalid literal kind"),
        };

        if self.cursor.consume_if(|c| c == '\n') {
            return TokenKind::Unknown;
        }

        self.cursor
            .push_while(&mut self.buffer, |c| c != quote && c != '\n');

        if !self.cursor.consume_if(|c| c == quote) {
            self.buffer.clear();
            return TokenKind::Unknown;
        }

        let symbol = Symbol::insert(&self.buffer);
        self.buffer.clear();

        Literal {
            kind,
            value: symbol,
        }
        .into()
    }

    #[inline]
    fn whitespace(&mut self) -> TokenKind {
        self.cursor.consume_while(chars::is_whitespace);
        TokenKind::Whitespace
    }
}

mod chars {
    #[inline]
    #[must_use]
    pub fn is_ident_start(c: char) -> bool {
        unicode_ident::is_xid_start(c) || c == '_'
    }

    #[inline]
    #[must_use]
    pub fn is_ident_continue(c: char) -> bool {
        unicode_ident::is_xid_continue(c) || c == '_'
    }

    #[inline]
    #[must_use]
    pub fn is_suffix_start(c: char) -> bool {
        unicode_ident::is_xid_start(c)
    }

    #[inline]
    #[must_use]
    pub fn is_decimal_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    #[inline]
    #[must_use]
    pub fn is_binary_digit(c: char) -> bool {
        c == '0' || c == '1'
    }

    #[inline]
    #[must_use]
    pub fn is_octal_digit(c: char) -> bool {
        ('0'..='8').contains(&c)
    }

    #[inline]
    #[must_use]
    pub fn is_hexadecimal_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    #[inline]
    #[must_use]
    pub fn is_whitespace(c: char) -> bool {
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
