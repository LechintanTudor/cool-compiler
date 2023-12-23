use crate::{ExprId, ParseResult, Parser, TyId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct AsExpr {
    pub span: Span,
    pub base: ExprId,
    pub ty: TyId,
}

impl Parser<'_> {
    pub fn continue_parse_as_expr(&mut self, base: ExprId) -> ParseResult<ExprId> {
        self.bump_expect(&tk::kw_as)?;

        let ty = self.parse_ty()?;
        let span = self[base].span().to(self[ty].span());

        Ok(self.add_expr(AsExpr { span, base, ty }))
    }
}
