use crate::lexer::{IdentTable, LineOffsets, LiteralTable, Token, TokenKind, Tokenizer};

pub struct SourceFile {
    pub name: String,
    pub source: String,
    pub line_offsets: LineOffsets,
    pub idents: IdentTable,
    pub literals: LiteralTable,
    pub tokens: Vec<Token>,
}

impl SourceFile {
    pub fn from_name_and_source(name: String, source: String) -> Self {
        let mut line_offsets = LineOffsets::default();
        let mut idents = IdentTable::default();
        let mut literals = LiteralTable::default();

        let mut tokenizer = Tokenizer::new(&source, &mut line_offsets, &mut idents, &mut literals);
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
            idents,
            literals,
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
