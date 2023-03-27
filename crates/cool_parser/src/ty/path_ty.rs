use crate::{Ident, IdentVec, ParseResult, ParseTree, Parser};
use cool_lexer::symbols::sym;
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct PathTy {
    pub idents: IdentVec,
}

impl ParseTree for PathTy {
    fn span(&self) -> Span {
        let start = self
            .idents
            .first()
            .map(|ident| ident.span)
            .unwrap_or(Span::empty());

        let end = self
            .idents
            .last()
            .map(|ident| ident.span)
            .unwrap_or(Span::empty());

        start.to(end)
    }
}

impl Parser<'_> {
    fn parse_ty_path_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();
        let symbol = match token.kind {
            TokenKind::Ident(symbol) => symbol,
            tk::KW_CRATE => sym::KW_CRATE,
            tk::KW_SUPER => sym::KW_SUPER,
            tk::KW_SELF => sym::KW_SELF,
            _ => {
                return self.error(
                    token,
                    &[tk::ANY_IDENT, tk::KW_CRATE, tk::KW_SUPER, tk::KW_SELF],
                )
            }
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }

    pub fn parse_path_ty(&mut self) -> ParseResult<PathTy> {
        let mut idents = IdentVec::new();
        idents.push(self.parse_ty_path_ident()?);

        while self.bump_if_eq(tk::DOT).is_some() {
            idents.push(self.parse_ty_path_ident()?);
        }

        Ok(PathTy { idents })
    }
}
