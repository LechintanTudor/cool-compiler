use crate::{Expr, ParseResult, Parser, Pattern, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub pattern: Pattern,
    pub ty: Option<Box<Ty>>,
    pub expr: Box<Expr>,
}

impl Section for DeclStmt {
    #[inline]
    #[must_use]
    fn span(&self) -> Span {
        self.pattern.span.to(self.expr.span())
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_decl_stmt(&mut self) -> ParseResult<DeclStmt> {
        let pattern = self.parse_pattern()?;
        self.continue_parse_decl_stmt(pattern)
    }

    pub fn continue_parse_decl_stmt(&mut self, pattern: Pattern) -> ParseResult<DeclStmt> {
        self.bump_expect(&tk::colon)?;

        let ty = (self.peek().kind != tk::eq)
            .then(|| self.parse_ty())
            .transpose()?
            .map(Box::new);

        self.bump_expect(&tk::eq)?;
        let expr = self.parse_expr()?;

        Ok(DeclStmt {
            pattern,
            ty,
            expr: Box::new(expr),
        })
    }
}
