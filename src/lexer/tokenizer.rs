use crate::lexer::{Cursor, Literal, LiteralKind, Punctuation, Token, TokenKind, EOF_CHAR};
use crate::symbol::SymbolTable;
use crate::utils::{LineOffsets, Span};

pub struct Tokenizer<'a> {
    cursor: Cursor<'a>,
    source_len: u32,
    line_offsets: &'a mut LineOffsets,
    symbols: &'a mut SymbolTable,
    buffer: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(
        source: &'a str,
        line_offsets: &'a mut LineOffsets,
        symbols: &'a mut SymbolTable,
    ) -> Self {
        Self {
            cursor: Cursor::from(source),
            source_len: source.len() as u32,
            line_offsets,
            symbols,
            buffer: Default::default(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let (offset, first_char) = self.cursor.bump_with_offset();

        let token_kind = if first_char == '/' && self.cursor.first() == '/' {
            self.cursor.bump();
            self.line_comment()
        } else if is_ident_start(first_char) {
            self.buffer.push(first_char);
            self.ident_or_keyword_or_bool_literal()
        } else if let Ok(start) = Punctuation::try_from(first_char) {
            self.punctuation(start)
        } else if ('0'..='9').contains(&first_char) {
            self.buffer.push(first_char);
            self.number()
        } else if first_char.is_whitespace() {
            if first_char == '\n' {
                self.line_offsets.add(self.cursor.offset());
            }
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

    fn ident_or_keyword_or_bool_literal(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| {
            if !is_ident_continue(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let symbol = self.symbols.insert(&self.buffer);

        let token = if symbol.is_keyword() {
            if symbol.is_bool_literal() {
                TokenKind::Literal(Literal {
                    kind: LiteralKind::Boolean,
                    symbol,
                })
            } else {
                TokenKind::Keyword(symbol)
            }
        } else {
            TokenKind::Ident(symbol)
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

    fn number(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| {
            if !(('0'..='9').contains(&char) || char == '_') {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let symbol = self.symbols.insert(&self.buffer);

        let suffix_start = self.buffer.len();
        self.cursor.consume_while(|char| {
            if char.is_whitespace() || is_punctuation(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let suffix = if suffix_start != self.buffer.len() {
            Some(self.symbols.insert(&self.buffer[suffix_start..]))
        } else {
            None
        };

        self.buffer.clear();

        TokenKind::Literal(Literal {
            kind: LiteralKind::Integer { suffix },
            symbol,
        })
    }

    fn whitespace(&mut self) -> TokenKind {
        loop {
            let char = self.cursor.first();

            if !char.is_whitespace() {
                break;
            }

            self.cursor.bump();

            if char == '\n' {
                self.line_offsets.add(self.cursor.offset());
            }
        }

        TokenKind::Whitespace
    }

    fn line_comment(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| char != '\n');

        if self.cursor.consume_if(|char| char == '\n') {
            self.line_offsets.add(self.cursor.offset());
        }

        TokenKind::Comment
    }
}

fn is_ident_start(char: char) -> bool {
    unicode_ident::is_xid_start(char) || char == '_'
}

fn is_ident_continue(char: char) -> bool {
    unicode_ident::is_xid_continue(char) || char == '_'
}

fn is_punctuation(char: char) -> bool {
    Punctuation::try_from(char).is_ok()
}
