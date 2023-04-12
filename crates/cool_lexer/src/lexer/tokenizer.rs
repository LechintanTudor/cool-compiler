use crate::lexer::{Cursor, TokenStream, EOF_CHAR};
use crate::symbols::Symbol;
use crate::tokens::{Literal, LiteralKind, Punctuation, Token, TokenKind};
use cool_span::Span;

pub struct Tokenizer<'a> {
    source: &'a str,
    cursor: Cursor<'a>,
    buffer: String,
}

impl<'a> Tokenizer<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: Cursor::from(source),
            buffer: Default::default(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.cursor = Cursor::from(self.source);
        self.buffer.clear();
    }

    #[inline]
    pub fn stream(&'a mut self) -> TokenStream<'a> {
        TokenStream::new(self)
    }

    #[inline]
    pub fn source(&self) -> &str {
        self.source
    }

    pub fn next_token(&mut self) -> Token {
        let (offset, first_char) = self.cursor.bump_with_offset();

        let token_kind = if first_char == '/' && self.cursor.first() == '/' {
            self.cursor.bump();
            self.line_comment()
        } else if is_ident_start(first_char) {
            self.buffer.push(first_char);
            self.ident()
        } else if let Ok(start) = Punctuation::try_from(first_char) {
            self.punctuation(start)
        } else if ('0'..='9').contains(&first_char) {
            self.buffer.push(first_char);

            match self.cursor.first() {
                'b' => {
                    self.buffer.push(self.cursor.bump());
                    self.binary_number()
                }
                'o' => {
                    self.buffer.push(self.cursor.bump());
                    self.octal_number()
                }
                'x' => {
                    self.buffer.push(self.cursor.bump());
                    self.hexadecimal_number()
                }
                _ => self.decimal_number(),
            }
        } else if first_char == '"' {
            self.string()
        } else if first_char == '\'' {
            self.character()
        } else if first_char.is_whitespace() {
            self.whitespace()
        } else if first_char == EOF_CHAR {
            TokenKind::Eof
        } else {
            TokenKind::Unknown
        };

        Token {
            span: Span::from_to(offset, self.cursor.offset()),
            kind: token_kind,
        }
    }

    fn ident(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| {
            if !is_ident_continue(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let symbol = Symbol::insert(&self.buffer);

        let token = if symbol.is_keyword() {
            if symbol.is_bool_literal() {
                TokenKind::Literal(Literal {
                    kind: LiteralKind::Bool,
                    symbol,
                })
            } else {
                TokenKind::Keyword(symbol)
            }
        } else {
            if can_have_prefix(self.cursor.first()) {
                TokenKind::Prefix(symbol)
            } else {
                TokenKind::Ident(symbol)
            }
        };

        self.buffer.clear();
        token
    }

    fn punctuation(&mut self, start: Punctuation) -> TokenKind {
        let mut punctuation = start;

        while let Ok(next) = Punctuation::try_from(self.cursor.first()) {
            match punctuation.join(next) {
                Ok(joined) => {
                    self.cursor.bump();
                    punctuation = joined;
                }
                Err(_) => break,
            }
        }

        TokenKind::Punctuation(punctuation)
    }

    fn binary_number(&mut self) -> TokenKind {
        self.number_literal(|c| ('0'..'1').contains(&c))
    }

    fn octal_number(&mut self) -> TokenKind {
        self.number_literal(|c| ('0'..'7').contains(&c))
    }

    fn decimal_number(&mut self) -> TokenKind {
        self.number_literal(|c| ('0'..'9').contains(&c))
    }

    fn hexadecimal_number(&mut self) -> TokenKind {
        self.number_literal(|c| {
            ('0'..'9').contains(&c) || ('a'..'f').contains(&c) || ('A'..'F').contains(&c)
        })
    }

    fn number_literal(&mut self, is_digit_allowed: impl Fn(char) -> bool) -> TokenKind {
        self.cursor.consume_while(|char| {
            if !(is_digit_allowed(char) || char == '_') {
                return false;
            }

            self.buffer.push(char);
            true
        });

        self.cursor.consume_if(|char| {
            if !is_ident_start(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        self.cursor.consume_while(|char| {
            if !is_ident_continue(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let symbol = Symbol::insert(&self.buffer);
        self.buffer.clear();

        TokenKind::Literal(Literal {
            kind: LiteralKind::Number,
            symbol,
        })
    }

    fn string(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| {
            if ['\n', '"'].contains(&char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let token = match self.cursor.bump() {
            '"' => TokenKind::Literal(Literal {
                kind: LiteralKind::Str,
                symbol: Symbol::insert(&self.buffer),
            }),
            _ => TokenKind::Unknown,
        };

        self.buffer.clear();
        token
    }

    fn character(&mut self) -> TokenKind {
        let char = self.cursor.bump();

        if char == '\'' {
            return TokenKind::Unknown;
        }

        self.buffer.push(char);
        let token = TokenKind::Literal(Literal {
            kind: LiteralKind::Char,
            symbol: Symbol::insert(&self.buffer),
        });

        self.buffer.clear();
        token
    }

    fn whitespace(&mut self) -> TokenKind {
        loop {
            let char = self.cursor.first();

            if !char.is_whitespace() {
                break;
            }

            self.cursor.bump();
        }

        TokenKind::Whitespace
    }

    fn line_comment(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| char != '\n');
        TokenKind::Comment
    }
}

fn is_ident_start(char: char) -> bool {
    unicode_ident::is_xid_start(char) || char == '_'
}

fn is_ident_continue(char: char) -> bool {
    unicode_ident::is_xid_continue(char) || char == '_'
}

fn can_have_prefix(char: char) -> bool {
    ['\'', '"'].contains(&char)
}
