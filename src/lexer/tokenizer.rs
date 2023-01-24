use crate::lexer::{
    Cursor, LineOffsets, Literal, LiteralKind, Operator, Separator, Token, TokenKind, EOF_CHAR,
};
use crate::symbol::SymbolTable;

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

        let token_kind = match first_char {
            '/' => match self.cursor.first() {
                '/' => {
                    self.cursor.bump();
                    self.line_comment()
                }
                _ => {
                    self.buffer.push(first_char);
                    self.operator()
                }
            },

            // Identifier or keyword
            _ if is_ident_start(first_char) => {
                self.buffer.push(first_char);
                self.ident_or_keyword()
            }

            // Operator
            _ if is_operator_part(first_char) => {
                self.buffer.push(first_char);
                self.operator()
            }

            // Separators
            ',' => Separator::Comma.into(),
            ';' => Separator::Semi.into(),
            ':' => Separator::Colon.into(),
            '(' => Separator::OpenParen.into(),
            ')' => Separator::ClosedParen.into(),
            '[' => Separator::OpenBracket.into(),
            ']' => Separator::ClosedBracket.into(),
            '{' => Separator::OpenBrace.into(),
            '}' => Separator::ClosedBrace.into(),

            // Numbers
            '0'..='9' => {
                self.buffer.push(first_char);
                self.number()
            }

            // Whitespace
            _ if first_char.is_whitespace() => {
                if first_char == '\n' {
                    self.line_offsets.add(self.cursor.offset());
                }

                self.whitespace()
            }

            // End of file
            EOF_CHAR => TokenKind::Eof,

            // If nothing else matches, return the unknown token
            _ => TokenKind::Unknown,
        };

        Token::new(offset, self.cursor.offset() - offset, token_kind)
    }

    fn ident_or_keyword(&mut self) -> TokenKind {
        self.cursor.consume_while(|char| {
            if !is_ident_continue(char) {
                return false;
            }

            self.buffer.push(char);
            true
        });

        let symbol = self.symbols.insert(&self.buffer);

        let token = if symbol.is_keyword() {
            TokenKind::Keyword(symbol)
        } else {
            TokenKind::Ident(symbol)
        };

        self.buffer.clear();
        token
    }

    fn operator(&mut self) -> TokenKind {
        let mut operator = Operator::try_from(self.buffer.as_str())
            .expect("all operator parts mut be valid operators");

        self.cursor.consume_while(|char| {
            if !is_operator_part(char) {
                return false;
            }

            self.buffer.push(char);

            match Operator::try_from(self.buffer.as_str()) {
                Ok(new_operator) => {
                    operator = new_operator;
                    true
                }
                Err(_) => false,
            }
        });

        self.buffer.clear();
        TokenKind::Operator(operator)
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
        self.buffer.clear();

        TokenKind::Literal(Literal {
            kind: LiteralKind::Integer,
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

pub fn is_operator_part(char: char) -> bool {
    const OPERATOR_PARTS: &[char] = &[
        '!', '%', '&', '*', '+', '-', '/', '<', '=', '>', '^', '|', '.',
    ];

    OPERATOR_PARTS.contains(&char)
}
