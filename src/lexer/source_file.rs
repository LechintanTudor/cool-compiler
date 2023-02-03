use crate::lexer::{Token, TokenKind, Tokenizer};
use crate::symbol::SYMBOL_TABLE;
use crate::utils::LineOffsets;

pub struct SourceFile {
    pub name: String,
    pub source: String,
    pub line_offsets: LineOffsets,
    pub tokens: Vec<Token>,
}

impl SourceFile {
    pub fn from_name_and_source(name: String, source: String) -> Self {
        let mut symbol_table = SYMBOL_TABLE.write_inner();
        let mut line_offsets = LineOffsets::default();
        let mut tokenizer = Tokenizer::new(&source, &mut line_offsets, &mut symbol_table);
        let mut tokens = Vec::<Token>::new();

        loop {
            let token = tokenizer.next_token();
            tokens.push(token);

            if token.kind == TokenKind::Eof {
                break;
            }
        }

        Self {
            name,
            source,
            line_offsets,
            tokens,
        }
    }

    pub fn iter_lang_tokens(&self) -> impl Iterator<Item = Token> + '_ {
        self.tokens
            .iter()
            .filter(|token| token.kind.is_lang_part())
            .copied()
    }
}
