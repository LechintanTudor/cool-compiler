use crate::{Expr, ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AsExpr {
    pub base: Box<Expr>,
    pub ty: Ty,
}

impl Section for AsExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ty.span())
    }
}

impl Parser<'_> {
    pub fn continue_parse_as_expr(&mut self, base: Expr) -> ParseResult<AsExpr> {
        self.bump_expect(&tk::kw_as)?;
        let ty = self.parse_ty()?;

        Ok(AsExpr {
            base: Box::new(base),
            ty,
        })
    }
}
