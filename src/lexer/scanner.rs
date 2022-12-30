use crate::lexer::{
    self, IdentifierTable, Keyword, Literal, LiteralTable, Operator, Separator, Token,
};

#[derive(Clone, Copy, Default, Debug)]
pub enum ScannerState {
    #[default]
    Default,
    Underscore,
    IdentifierOrKeyword,
    Integer,
    String,
    StringEscape,
    Operator,
}

#[derive(Default)]
pub struct Scanner {
    state: ScannerState,
    buffer: Vec<u8>,
    tokens: Vec<Token>,
    identifier_table: IdentifierTable,
    literal_table: LiteralTable,
}

impl Scanner {
    pub fn consume(&mut self, byte: u8) -> anyhow::Result<()> {
        match self.state {
            ScannerState::Default => self.consume_from_default(byte)?,
            ScannerState::Underscore => self.consume_from_underscore(byte)?,
            ScannerState::IdentifierOrKeyword => self.consume_from_identifier_or_keyword(byte)?,
            ScannerState::Integer => self.consume_from_integer(byte)?,
            ScannerState::String => self.consume_from_string(byte)?,
            ScannerState::StringEscape => self.consume_from_string_escape(byte)?,
            ScannerState::Operator => self.consume_from_operator(byte)?,
        }

        Ok(())
    }

    pub fn into_program(self) -> (Vec<Token>, IdentifierTable, LiteralTable) {
        (self.tokens, self.identifier_table, self.literal_table)
    }

    fn consume_from_default(&mut self, byte: u8) -> anyhow::Result<()> {
        debug_assert!(self.buffer.is_empty());

        if byte == b'_' {
            self.buffer.push(byte);
            self.state = ScannerState::Underscore;
        } else if byte == b'"' {
            self.state = ScannerState::String;
        } else if byte.is_ascii_alphabetic() {
            self.buffer.push(byte);
            self.state = ScannerState::IdentifierOrKeyword;
        } else if byte.is_ascii_digit() {
            self.buffer.push(byte);
            self.state = ScannerState::Integer;
        } else if lexer::is_operator_part(byte) {
            self.buffer.push(byte);
            self.state = ScannerState::Operator;
        } else if let Ok(separator) = Separator::try_from(byte) {
            self.tokens.push(separator.into());
        } else if byte.is_ascii_whitespace() {
            // Skip..
        } else {
            panic!("lexical error");
        }

        Ok(())
    }

    fn consume_from_underscore(&mut self, byte: u8) -> anyhow::Result<()> {
        debug_assert_eq!(self.buffer.as_slice(), b"_");

        if lexer::is_identifier_middle_part(byte) {
            self.state = ScannerState::IdentifierOrKeyword;
            self.buffer.push(byte);
        } else if lexer::is_operator_part(byte) {
            self.consume_buffer_as_wildcard();
            self.buffer.push(byte);
            self.state = ScannerState::Operator;
        } else if let Ok(separator) = Separator::try_from(byte) {
            self.consume_buffer_as_wildcard();
            self.tokens.push(separator.into());
            self.state = ScannerState::Default;
        } else if byte.is_ascii_whitespace() {
            self.consume_buffer_as_wildcard();
            self.state = ScannerState::Default;
        } else {
            panic!("lexical error");
        }

        Ok(())
    }

    fn consume_from_identifier_or_keyword(&mut self, byte: u8) -> anyhow::Result<()> {
        if lexer::is_identifier_middle_part(byte) {
            self.buffer.push(byte);
        } else if lexer::is_operator_part(byte) {
            self.consume_buffer_as_identifier_or_keyword();
            self.buffer.push(byte);
            self.state = ScannerState::Operator;
        } else if let Ok(separator) = Separator::try_from(byte) {
            self.consume_buffer_as_identifier_or_keyword();
            self.tokens.push(separator.into());
            self.state = ScannerState::Default;
        } else if byte.is_ascii_whitespace() {
            self.consume_buffer_as_identifier_or_keyword();
            self.state = ScannerState::Default;
        } else {
            panic!("lexical error");
        }

        Ok(())
    }

    fn consume_from_integer(&mut self, byte: u8) -> anyhow::Result<()> {
        if byte.is_ascii_digit() {
            self.buffer.push(byte);
        } else if let Ok(separator) = Separator::try_from(byte) {
            self.consume_buffer_as_integer();
            self.tokens.push(separator.into());
            self.state = ScannerState::Default;
        } else if lexer::is_operator_part(byte) {
            self.consume_buffer_as_integer();
            self.buffer.push(byte);
            self.state = ScannerState::Operator;
        } else if byte.is_ascii_whitespace() {
            self.consume_buffer_as_integer();
            self.state = ScannerState::Default;
        } else {
            panic!("lexical error");
        }

        Ok(())
    }

    fn consume_from_string(&mut self, byte: u8) -> anyhow::Result<()> {
        if byte == b'"' {
            self.consume_buffer_as_string();
            self.state = ScannerState::Default;
        } else if byte == b'\\' {
            self.state = ScannerState::StringEscape;
        } else {
            self.buffer.push(byte);
        }

        Ok(())
    }

    fn consume_from_string_escape(&mut self, byte: u8) -> anyhow::Result<()> {
        match byte {
            b'\\' | b'"' => self.buffer.push(byte),
            b'n' => self.buffer.push(b'\n'),
            b'0' => self.buffer.push(b'\0'),
            _ => panic!("lexical error"),
        }

        self.state = ScannerState::String;
        Ok(())
    }

    fn consume_from_operator(&mut self, byte: u8) -> anyhow::Result<()> {
        if byte == b'_' {
            self.consume_buffer_as_operators()?;
            self.buffer.push(byte);
            self.state = ScannerState::Underscore;
        } else if byte == b'"' {
            self.consume_buffer_as_operators()?;
            self.state = ScannerState::String;
        } else if byte.is_ascii_alphabetic() {
            self.consume_buffer_as_operators()?;
            self.buffer.push(byte);
            self.state = ScannerState::IdentifierOrKeyword;
        } else if byte.is_ascii_digit() {
            self.consume_buffer_as_operators()?;
            self.buffer.push(byte);
            self.state = ScannerState::Integer;
        } else if lexer::is_operator_part(byte) {
            self.buffer.push(byte);
        } else if let Ok(separator) = Separator::try_from(byte) {
            self.consume_buffer_as_operators()?;
            self.tokens.push(separator.into());
            self.state = ScannerState::Default;
        } else if byte.is_ascii_whitespace() {
            self.consume_buffer_as_operators()?;
            self.state = ScannerState::Default;
        } else {
            panic!("lexical error");
        }

        Ok(())
    }

    fn consume_buffer_as_wildcard(&mut self) {
        debug_assert_eq!(self.buffer.as_slice(), b"_");

        self.buffer.clear();
        self.tokens.push(Token::Wildcard);
    }

    fn consume_buffer_as_identifier_or_keyword(&mut self) {
        match Keyword::try_from(self.buffer.as_slice()) {
            Ok(keyword) => {
                self.tokens.push(keyword.into());
            }
            Err(_) => {
                let identifier = std::str::from_utf8(&self.buffer).unwrap();
                let index = self.identifier_table.insert(identifier);
                self.tokens.push(Token::Ident { index })
            }
        }

        self.buffer.clear();
    }

    fn consume_buffer_as_integer(&mut self) {
        let literal = String::from_utf8(self.buffer.clone()).unwrap();
        let index = self.literal_table.insert(Literal::Integer(literal));
        self.tokens.push(Token::Literal { index });
        self.buffer.clear();
    }

    fn consume_buffer_as_string(&mut self) {
        let literal = String::from_utf8(self.buffer.clone()).unwrap();
        let index = self.literal_table.insert(Literal::String(literal));
        self.tokens.push(Token::Literal { index });
        self.buffer.clear();
    }

    fn consume_buffer_as_operators(&mut self) -> anyhow::Result<()> {
        let mut start = 0;
        let mut len = self.buffer.len();

        while start < self.buffer.len() {
            let end = start + len;

            match Operator::try_from(&self.buffer[start..end]) {
                Ok(operator) => {
                    self.tokens.push(Token::Operator(operator));
                    start += len;
                    len = self.buffer.len() - len;
                }
                Err(_) => {
                    if len == 1 {
                        panic!("lexer error");
                    }

                    len -= 1;
                }
            }
        }

        self.buffer.clear();
        Ok(())
    }
}
