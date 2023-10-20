use crate::{Decl, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::TokenKind;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct SourceFile {
    pub span: Span,
    pub decls: Vec<Decl>,
}

impl Parser<'_> {
    pub fn parse_source_file(&mut self) -> ParseResult<SourceFile> {
        let mut decls = Vec::<Decl>::new();

        let span = loop {
            if let Some(eof) = self.bump_if_eq(TokenKind::Eof) {
                break Span {
                    start: 0,
                    len: eof.span.start,
                };
            }

            decls.push(self.parse_decl()?);
        };

        Ok(SourceFile { span, decls })
    }
}
