use crate::{Expr, ExprOrStmt, ParseResult, Parser, Pattern, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct MatchArm {
    pub span: Span,
    pub ty: Box<Ty>,
    pub pattern: Option<Pattern>,
    pub code: Box<ExprOrStmt>,
    pub has_trailing_comma: bool,
}

impl Section for MatchArm {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct ElseArm {
    pub span: Span,
    pub code: Box<ExprOrStmt>,
    pub has_trailing_comma: bool,
}

impl Section for ElseArm {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct MatchExpr {
    pub span: Span,
    pub expr: Box<Expr>,
    pub arms: Vec<MatchArm>,
    pub else_arm: Option<ElseArm>,
}

impl Section for MatchExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_match_expr(&mut self) -> ParseResult<MatchExpr> {
        let open_brace = self.bump_expect(&tk::KW_MATCH)?;
        let expr = self.parse_non_struct_expr()?;
        self.bump_expect(&tk::OPEN_BRACE)?;

        let mut arms = Vec::<MatchArm>::new();

        let (close_brace, else_arm) = loop {
            if let Some(close_brace) = self.bump_if_eq(tk::CLOSE_BRACE) {
                break (close_brace, None);
            }

            if self.peek().kind == tk::KW_ELSE {
                let else_arm = self.parse_else_arm()?;
                let close_brace = self.bump_expect(&tk::CLOSE_BRACE)?;
                break (close_brace, Some(else_arm));
            }

            arms.push(self.parse_match_arm()?);
        };

        Ok(MatchExpr {
            span: open_brace.span.to(close_brace.span),
            expr: Box::new(expr),
            arms,
            else_arm,
        })
    }

    fn parse_match_arm(&mut self) -> ParseResult<MatchArm> {
        let ty = self.parse_ty()?;
        let pattern = self
            .bump_if_eq(tk::KW_AS)
            .map(|_| self.parse_pattern())
            .transpose()?;

        self.bump_expect(&tk::FAT_ARROW)?;
        let code = self.parse_bare_expr_or_stmt(false, true)?;

        let (end_span, has_trailing_comma) = match self.bump_if_eq(tk::COMMA) {
            Some(trailing_comma) => (trailing_comma.span, true),
            None => {
                if !code.is_promotable_to_stmt() && self.peek().kind != tk::CLOSE_BRACE {
                    self.peek_error(&[tk::COMMA])?;
                }

                (code.span(), false)
            }
        };

        Ok(MatchArm {
            span: ty.span().to(end_span),
            ty: Box::new(ty),
            pattern,
            code: Box::new(code),
            has_trailing_comma,
        })
    }

    fn parse_else_arm(&mut self) -> ParseResult<ElseArm> {
        let start_token = self.bump_expect(&tk::KW_ELSE)?;

        self.bump_expect(&tk::FAT_ARROW)?;
        let code = self.parse_bare_expr_or_stmt(false, true)?;

        let (end_span, has_trailing_comma) = match self.bump_if_eq(tk::COMMA) {
            Some(trailing_comma) => (trailing_comma.span, true),
            None => (code.span(), false),
        };

        Ok(ElseArm {
            span: start_token.span.to(end_span),
            code: Box::new(code),
            has_trailing_comma,
        })
    }
}
