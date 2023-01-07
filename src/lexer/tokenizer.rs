use crate::lexer::{
    self, IdentTable, Keyword, LiteralTable, Separator, SourceCharIter, SpannedToken, Token,
};
use std::error::Error;
use std::fmt;

type NextTokenResult = Result<Option<SpannedToken>, TokenizerError>;

#[derive(Clone, Debug)]
pub struct TokenizerError {
    pub offset: u32,
    pub kind: TokenizerErrorKind,
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenizerErrorKind::SourceTooLong => {
                write!(f, "source file is too long")
            }
            TokenizerErrorKind::UnexpectedChar { char } => {
                write!(f, "unexpected char {} at position {}", char, self.offset)
            }
            TokenizerErrorKind::UnexpectedEof => {
                write!(f, "unexepected end of file")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenizerErrorKind {
    SourceTooLong,
    UnexpectedChar { char: char },
    UnexpectedEof,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum TokenizerState {
    #[default]
    Default,
    Underscore,
    IdentOrKeyword,
    Integer,
    String,
    StringEscape,
    Operator,
    Separator(Separator),
    Eof,
    Terminated,
}

pub struct Tokenizer<'a> {
    chars: SourceCharIter<'a>,
    source_len: u32,
    state: TokenizerState,
    terminated: bool,
    token_start: u32,
    buffer: String,
    line_offsets: &'a mut Vec<u32>,
    idents: &'a mut IdentTable,
    literals: &'a mut LiteralTable,
}

impl<'a> Tokenizer<'a> {
    pub fn new(
        source: &'a str,
        line_offsets: &'a mut Vec<u32>,
        idents: &'a mut IdentTable,
        literals: &'a mut LiteralTable,
    ) -> Result<Self, TokenizerError> {
        if source.len() >= u32::MAX as usize {
            return Err(TokenizerError {
                offset: 0,
                kind: TokenizerErrorKind::SourceTooLong,
            });
        }

        Ok(Self {
            chars: SourceCharIter::from(source),
            source_len: source.len() as u32,
            state: Default::default(),
            terminated: false,
            token_start: 0,
            buffer: String::new(),
            line_offsets,
            idents,
            literals,
        })
    }

    fn next_from_default(&mut self, offset: u32, char: char) -> NextTokenResult {
        debug_assert_eq!(self.state, TokenizerState::Default);

        Ok(if char == '_' {
            self.state = TokenizerState::Underscore;
            self.token_start = offset;
            self.buffer.push('_');
            None
        } else if char == '"' {
            self.state = TokenizerState::String;
            self.token_start = offset;
            None
        } else if let Ok(separator) = Separator::try_from(char) {
            Some(SpannedToken::separator(offset, separator))
        } else if lexer::is_operator_part(char) {
            self.state = TokenizerState::Operator;
            self.token_start = offset;
            self.buffer.push(char);
            None
        } else if unicode_ident::is_xid_start(char) {
            self.state = TokenizerState::IdentOrKeyword;
            self.token_start = offset;
            self.buffer.push(char);
            None
        } else if char.is_whitespace() {
            None
        } else {
            return Err(TokenizerError {
                offset,
                kind: TokenizerErrorKind::UnexpectedChar { char },
            });
        })
    }

    fn next_from_underscore(&mut self, offset: u32, char: char) -> NextTokenResult {
        debug_assert_eq!(self.state, TokenizerState::Underscore);

        Ok(if lexer::is_ident_continue(char) {
            self.state = TokenizerState::IdentOrKeyword;
            self.buffer.push(char);
            None
        } else if lexer::is_operator_part(char) {
            let token = self.consume_buffer_as_underscore();
            self.state = TokenizerState::Operator;
            self.token_start = offset;
            self.buffer.push(char);
            Some(token)
        } else if let Ok(separator) = Separator::try_from(char) {
            let token = self.consume_buffer_as_underscore();
            self.set_separator(offset, separator);
            Some(token)
        } else if char.is_whitespace() {
            let token = self.consume_buffer_as_underscore();
            self.state = TokenizerState::Default;
            Some(token)
        } else {
            return Err(TokenizerError {
                offset,
                kind: TokenizerErrorKind::UnexpectedChar { char },
            });
        })
    }

    fn next_from_ident_or_keyword(&mut self, offset: u32, char: char) -> NextTokenResult {
        debug_assert_eq!(self.state, TokenizerState::IdentOrKeyword);

        Ok(if lexer::is_ident_continue(char) {
            self.buffer.push(char);
            None
        } else if lexer::is_operator_part(char) {
            let token = self.consume_buffer_as_ident_or_keyword(offset);
            self.state = TokenizerState::Operator;
            self.token_start = offset;
            self.buffer.push(char);
            Some(token)
        } else if let Ok(separator) = Separator::try_from(char) {
            let token = self.consume_buffer_as_ident_or_keyword(offset);
            self.set_separator(offset, separator);
            Some(token)
        } else if char.is_whitespace() {
            let token = self.consume_buffer_as_ident_or_keyword(offset);
            self.state = TokenizerState::Default;
            Some(token)
        } else {
            return Err(TokenizerError {
                offset,
                kind: TokenizerErrorKind::UnexpectedChar { char },
            });
        })
    }

    fn next_before_eof(&mut self) -> NextTokenResult {
        Ok(match self.state {
            TokenizerState::Default => None,
            TokenizerState::Underscore => {
                Some(SpannedToken::new(self.token_start, 1, Token::Underscore))
            }
            TokenizerState::IdentOrKeyword => {
                Some(self.consume_buffer_as_ident_or_keyword(self.source_len))
            }
            TokenizerState::String => {
                return Err(TokenizerError {
                    offset: self.source_len,
                    kind: TokenizerErrorKind::UnexpectedEof,
                })
            }
            _ => todo!(),
        })
    }

    fn set_separator(&mut self, offset: u32, separator: Separator) {
        self.token_start = offset;
        self.state = TokenizerState::Separator(separator);
    }

    fn consume_buffer_as_underscore(&mut self) -> SpannedToken {
        debug_assert_eq!(self.state, TokenizerState::Underscore);

        self.buffer.clear();
        SpannedToken::new(self.token_start, 1, Token::Underscore)
    }

    fn consume_buffer_as_ident_or_keyword(&mut self, offset: u32) -> SpannedToken {
        debug_assert_eq!(self.state, TokenizerState::IdentOrKeyword);

        let token = match Keyword::try_from(self.buffer.as_str()) {
            Ok(keyword) => Token::Keyword(keyword),
            Err(_) => Token::Ident {
                index: self.idents.insert(&self.buffer),
            },
        };

        self.buffer.clear();
        SpannedToken::new(self.token_start, offset - self.token_start, token)
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<SpannedToken, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                TokenizerState::Separator(separator) => {
                    self.state = TokenizerState::Default;
                    return Some(Ok(SpannedToken::separator(self.token_start, separator)));
                }
                TokenizerState::Eof => {
                    self.state = TokenizerState::Terminated;
                    return Some(Ok(SpannedToken::new(self.source_len, 0, Token::Eof)));
                }
                TokenizerState::Terminated => {
                    return None;
                }
                _ => (),
            }

            let Some((offset, char)) = self.chars.next() else {
                let token_result = self.next_before_eof();
                self.state = TokenizerState::Eof;
                
                match token_result {
                    Ok(Some(token)) => break Some(Ok(token)),
                    Err(error) => break Some(Err(error)),
                    _ => continue,
                }
            };

            if char == '\n' {
                self.line_offsets.push(offset);
            }

            let token_result = match self.state {
                TokenizerState::Default => self.next_from_default(offset, char),
                TokenizerState::Underscore => self.next_from_underscore(offset, char),
                TokenizerState::IdentOrKeyword => self.next_from_ident_or_keyword(offset, char),
                _ => continue,
            };

            match token_result {
                Ok(Some(token)) => break Some(Ok(token)),
                Err(error) => break Some(Err(error)),
                _ => continue,
            }
        }
    }
}
