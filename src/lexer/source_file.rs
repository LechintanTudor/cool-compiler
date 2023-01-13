use crate::lexer::{LineOffsets, Token, TokenKind, Tokenizer};
use crate::symbol::SymbolTable;

pub struct SourceFile {
    pub name: String,
    pub source: String,
    pub line_offsets: LineOffsets,
    pub symbols: SymbolTable,
    pub tokens: Vec<Token>,
}

impl SourceFile {
    pub fn from_name_and_source(name: String, source: String) -> Self {
        let mut line_offsets = LineOffsets::default();
        let mut symbols = SymbolTable::default();

        let mut tokenizer = Tokenizer::new(&source, &mut line_offsets, &mut symbols);
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
            symbols,
            tokens,
        }
    }

    pub fn iter_all_tokens(&self) -> impl Iterator<Item = &Token> + '_ {
        self.tokens.iter()
    }

    pub fn iter_lang_tokens(&self) -> impl Iterator<Item = &Token> + '_ {
        self.tokens.iter().filter(|token| token.kind.is_lang_part())
    }
}
