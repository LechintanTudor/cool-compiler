use crate::expr::Expr;
use crate::ty::Ty;
use crate::{ParseResult, Parser, Pattern};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub ty: Option<Box<Ty>>,
    pub expr: Box<Expr>,
}

impl Section for DeclStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_decl_stmt(&mut self) -> ParseResult<DeclStmt> {
        let pattern = self.parse_pattern()?;
        self.continue_parse_decl(pattern)
    }

    pub fn continue_parse_decl(&mut self, pattern: Pattern) -> ParseResult<DeclStmt> {
        self.bump_expect(&tk::COLON)?;

        let ty = if self.peek().kind != tk::EQ {
            Some(self.parse_ty()?)
        } else {
            None
        };

        self.bump_expect(&tk::EQ)?;

        let expr = self.parse_expr()?;
        let end_token = self.bump_expect(&tk::SEMICOLON)?;

        Ok(DeclStmt {
            span: pattern.span().to(end_token.span),
            pattern,
            ty: ty.map(Box::new),
            expr: Box::new(expr),
        })
    }
}
