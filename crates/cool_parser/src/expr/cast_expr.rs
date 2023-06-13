use crate::{Expr, ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CastExpr {
    pub base: Box<Expr>,
    pub ty: Box<Ty>,
}

impl Section for CastExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ty.span())
    }
}

impl Parser<'_> {
    pub fn continue_parse_cast_expr(&mut self, base: Box<Expr>) -> ParseResult<CastExpr> {
        self.bump_expect(&tk::KW_AS)?;
        let ty = self.parse_ty()?;

        Ok(CastExpr {
            base,
            ty: Box::new(ty),
        })
    }
}
