use crate::lexer::{IdentTable, LiteralTable, SpannedToken, Token, Tokenizer, TokenizerError};

pub struct SourceFile {
    pub name: String,
    pub source: String,
    pub line_offsets: Vec<u32>,
    pub idents: IdentTable,
    pub literals: LiteralTable,
    pub spanned_tokens: Vec<SpannedToken>,
}

impl SourceFile {
    pub fn from_name_and_source(name: String, source: String) -> Result<Self, TokenizerError> {
        let mut line_offsets = Vec::<u32>::new();
        let mut idents = IdentTable::default();
        let mut literals = LiteralTable::default();

        let tokenizer = Tokenizer::new(&source, &mut line_offsets, &mut idents, &mut literals)?;
        let spanned_tokens = tokenizer.collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name,
            source,
            line_offsets,
            idents,
            literals,
            spanned_tokens,
        })
    }

    pub fn iter_tokens(&self) -> impl Iterator<Item = Token> + '_ {
        self.spanned_tokens
            .iter()
            .map(|spanned_token| spanned_token.token)
    }
}
