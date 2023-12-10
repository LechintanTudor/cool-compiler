use crate::{ExprId, ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ForExpr {
    pub span: Span,
    pub decl: StmtId,
    pub cond: ExprId,
    pub after: StmtId,
    pub body: ExprId,
}

impl Parser<'_> {
    pub fn parse_for_expr(&mut self) -> ParseResult<ExprId> {
        let for_token = self.bump_expect(&tk::kw_for)?;

        let decl = self.parse_decl_stmt(true)?;
        self.bump_expect(&tk::semicolon)?;

        let cond = self.parse_expr()?;
        self.bump_expect(&tk::semicolon)?;

        let after = self.parse_stmt(false)?;
        let body = self.parse_block_expr()?;

        let span = for_token.span.to(self[body].span());

        Ok(self.add_expr(ForExpr {
            span,
            decl,
            cond,
            after,
            body,
        }))
    }
}
