use crate::{Expr, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct IndexExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub index: Box<Expr>,
}

impl Section for IndexExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub enum RangeKind {
    Full,
    From(Box<Expr>),
    To(Box<Expr>),
    FromTo((Box<Expr>, Box<Expr>)),
}

#[derive(Clone, Debug)]
pub struct RangeExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub is_mutable: bool,
    pub kind: RangeKind,
}

impl Section for RangeExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_subscript_expr(&mut self, base: Box<Expr>) -> ParseResult<Expr> {
        self.bump_expect(&tk::OPEN_BRACKET)?;
        let peeked_kind = self.peek().kind;

        if peeked_kind == tk::KW_MUT {
            return self
                .continue_parse_mut_range_expr(base)
                .map(RangeExpr::into);
        }

        if peeked_kind == tk::DOT_DOT {
            return self
                .continue_parse_range_to_expr(base, false)
                .map(RangeExpr::into);
        }

        let from = self.parse_expr()?;

        if let Some(close_bracket) = self.bump_if_eq(tk::CLOSE_BRACKET) {
            return Ok(Expr::Index(IndexExpr {
                span: base.span().to(close_bracket.span),
                base,
                index: Box::new(from),
            }));
        }

        self.continue_parse_range_from_expr(base, false, Box::new(from))
            .map(RangeExpr::into)
    }

    fn continue_parse_mut_range_expr(&mut self, base: Box<Expr>) -> ParseResult<RangeExpr> {
        self.bump_expect(&tk::KW_MUT)?;

        if self.peek().kind == tk::DOT_DOT {
            return self.continue_parse_range_to_expr(base, true);
        }

        let from = self.parse_expr()?;
        self.continue_parse_range_from_expr(base, true, Box::new(from))
    }

    fn continue_parse_range_to_expr(
        &mut self,
        base: Box<Expr>,
        is_mutable: bool,
    ) -> ParseResult<RangeExpr> {
        self.bump_expect(&tk::DOT_DOT)?;

        if let Some(close_bracket) = self.bump_if_eq(tk::CLOSE_BRACKET) {
            return Ok(RangeExpr {
                span: base.span().to(close_bracket.span),
                base,
                is_mutable,
                kind: RangeKind::Full,
            });
        }

        let to = self.parse_expr()?;
        let close_bracket = self.bump_expect(&tk::CLOSE_BRACKET)?;

        Ok(RangeExpr {
            span: base.span().to(close_bracket.span),
            base,
            is_mutable,
            kind: RangeKind::To(Box::new(to)),
        })
    }

    fn continue_parse_range_from_expr(
        &mut self,
        base: Box<Expr>,
        is_mutable: bool,
        from: Box<Expr>,
    ) -> ParseResult<RangeExpr> {
        self.bump_expect(&tk::DOT_DOT)?;

        if let Some(close_bracket) = self.bump_if_eq(tk::CLOSE_BRACKET) {
            return Ok(RangeExpr {
                span: base.span().to(close_bracket.span),
                base,
                is_mutable,
                kind: RangeKind::From(from),
            });
        }

        let to = self.parse_expr()?;
        let close_bracket = self.bump_expect(&tk::CLOSE_BRACKET)?;

        Ok(RangeExpr {
            span: base.span().to(close_bracket.span),
            base,
            is_mutable,
            kind: RangeKind::FromTo((from, Box::new(to))),
        })
    }
}
