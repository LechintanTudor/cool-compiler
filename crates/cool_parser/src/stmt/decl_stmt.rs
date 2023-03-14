use crate::expr::Expr;
use crate::ty::Ty;
use crate::{ParseResult, ParseTree, Parser, Pattern};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub ty: Option<Ty>,
    pub expr: Expr,
}

impl ParseTree for DeclStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_decl_stmt(&mut self) -> ParseResult<DeclStmt> {
        let pattern = self.parse_pattern()?;
        self.continue_parse_decl_after_pattern(pattern)
    }

    pub fn continue_parse_decl_after_pattern(&mut self, pattern: Pattern) -> ParseResult<DeclStmt> {
        self.bump_expect(&[tk::COLON])?;

        let ty = if self.peek().kind != tk::EQ {
            Some(self.parse_ty()?)
        } else {
            None
        };

        self.bump_expect(&[tk::EQ])?;

        let expr = self.parse_expr()?;
        let end_token = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(DeclStmt {
            span: pattern.span().to(end_token.span),
            pattern,
            ty,
            expr,
        })
    }
}
