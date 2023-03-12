use crate::expr::Expr;
use crate::path::IdentVec;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct PathExpr {
    pub root: Box<Expr>,
    pub idents: IdentVec,
}

impl ParseTree for PathExpr {
    fn span(&self) -> Span {
        let start_span = self.root.span();
        let end_span = self
            .idents
            .last()
            .map(|ident| ident.span)
            .unwrap_or(start_span);

        start_span.to(end_span)
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn continue_parse_path_expr(&mut self, root: Box<Expr>) -> ParseResult<PathExpr> {
        let mut idents = IdentVec::new();

        while self.bump_if_eq(tk::DOT).is_some() {
            idents.push(self.parse_ident()?);
        }

        Ok(PathExpr { root, idents })
    }
}
