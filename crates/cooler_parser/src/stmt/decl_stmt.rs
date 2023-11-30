use crate::{ExprId, ParseResult, Parser, Pattern, StmtId, TyId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub ty: Option<TyId>,
    pub expr: ExprId,
}

impl Parser<'_> {
    pub fn parse_decl_stmt(&mut self) -> ParseResult<StmtId> {
        let pattern = self.parse_pattern()?;
        self.continue_parse_decl_stmt(pattern)
    }

    pub fn continue_parse_decl_stmt(&mut self, pattern: Pattern) -> ParseResult<StmtId> {
        self.bump_expect(&tk::colon)?;

        let ty = (self.peek().kind != tk::colon)
            .then(|| self.parse_ty())
            .transpose()?;

        self.bump_expect(&tk::eq)?;

        let expr = self.parse_expr()?;
        let end_span = self[expr].span();

        Ok(self.add_stmt(DeclStmt {
            span: pattern.span.to(end_span),
            pattern,
            ty,
            expr,
        }))
    }
}
