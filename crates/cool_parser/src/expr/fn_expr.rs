use crate::{BlockExpr, FnAbiDecl, ParseResult, Parser, Pattern, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnExpr {
    pub span: Span,
    pub abi_decl: Option<FnAbiDecl>,
    pub params: FnExprParams,
    pub return_ty: Option<Box<Ty>>,
    pub body: Option<Box<BlockExpr>>,
}

#[derive(Clone, Section, Debug)]
pub struct FnExprParams {
    pub span: Span,
    pub params: Vec<FnExprParam>,
    pub is_variadic: bool,
    pub has_trailing_comma: bool,
}

#[derive(Clone, Section, Debug)]
pub struct FnExprParam {
    pub span: Span,
    pub pattern: Pattern,
    pub ty: Option<Ty>,
}

impl Parser<'_> {
    pub fn parse_fn_expr(&mut self) -> ParseResult<FnExpr> {
        let abi_decl = (self.peek().kind == tk::kw_extern)
            .then(|| self.parse_fn_abi_decl())
            .transpose()?;

        self.bump_expect(&tk::kw_fn)?;
        let params = self.parse_fn_expr_params()?;

        let return_ty = self
            .bump_if_eq(tk::arrow)
            .map(|_| self.parse_ty())
            .transpose()?;

        let body = (self.peek().kind == tk::open_brace)
            .then(|| self.parse_block_expr())
            .transpose()?;

        let start_span = abi_decl.as_ref().map(|abi| abi.span).unwrap_or(params.span);

        let end_span = body
            .as_ref()
            .map(|body| body.span)
            .or(return_ty.as_ref().map(|ty| ty.span()))
            .unwrap_or(params.span());

        Ok(FnExpr {
            span: start_span.to(end_span),
            abi_decl,
            params,
            return_ty: return_ty.map(Box::new),
            body: body.map(Box::new),
        })
    }

    fn parse_fn_expr_params(&mut self) -> ParseResult<FnExprParams> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut params = Vec::<FnExprParam>::new();

        let (close_paren, is_variadic, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false, false)
            } else {
                loop {
                    if self.bump_if_eq(tk::dot_dot_dot).is_some() {
                        break (self.bump_expect(&tk::close_paren)?, true, false);
                    }

                    params.push(self.parse_fn_expr_param()?);

                    let next_token = self.bump();

                    match next_token.kind {
                        tk::close_paren => {
                            break (next_token, false, false);
                        }
                        tk::comma => {
                            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                                break (close_paren, false, true);
                            }
                        }
                        _ => return self.error(next_token, &[tk::close_paren, tk::comma]),
                    }
                }
            };

        Ok(FnExprParams {
            span: open_paren.span.to(close_paren.span),
            params,
            is_variadic,
            has_trailing_comma,
        })
    }

    fn parse_fn_expr_param(&mut self) -> ParseResult<FnExprParam> {
        let pattern = self.parse_pattern()?;

        let ty = self
            .bump_if_eq(tk::colon)
            .map(|_| self.parse_ty())
            .transpose()?;

        let end_span = ty.as_ref().map(|ty| ty.span()).unwrap_or(pattern.span);

        Ok(FnExprParam {
            span: pattern.span.to(end_span),
            pattern,
            ty,
        })
    }
}
