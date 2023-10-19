use crate::{FnAbi, ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnTy {
    pub span: Span,
    pub abi: Option<FnAbi>,
    pub params: FnTyParams,
    pub return_ty: Option<Box<Ty>>,
}

#[derive(Clone, Section, Debug)]
pub struct FnTyParams {
    pub span: Span,
    pub param_tys: Vec<Ty>,
    pub is_variadic: bool,
    pub has_trailing_comma: bool,
}

impl Parser<'_> {
    pub fn parse_fn_ty(&mut self) -> ParseResult<FnTy> {
        let abi = (self.peek().kind == tk::kw_extern)
            .then(|| self.parse_fn_abi())
            .transpose()?;

        let fn_token = self.bump_expect(&tk::kw_fn)?;

        let params = self.parse_fn_ty_params()?;

        let return_ty = self
            .bump_if_eq(tk::arrow)
            .map(|_| self.parse_ty())
            .transpose()?
            .map(Box::new);

        let start_span = abi.as_ref().map(|abi| abi.span).unwrap_or(fn_token.span);

        let end_span = return_ty
            .as_ref()
            .map(|ty| ty.span())
            .unwrap_or(params.span);

        Ok(FnTy {
            span: start_span.to(end_span),
            abi,
            params,
            return_ty,
        })
    }

    pub fn parse_fn_ty_params(&mut self) -> ParseResult<FnTyParams> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut param_tys = Vec::<Ty>::new();

        let (close_paren, is_variadic, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false, false)
            } else {
                loop {
                    if self.bump_if_eq(tk::dot_dot_dot).is_some() {
                        break (self.bump_expect(&tk::close_paren)?, true, false);
                    }

                    param_tys.push(self.parse_ty()?);

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

        Ok(FnTyParams {
            span: open_paren.span.to(close_paren.span),
            param_tys,
            is_variadic,
            has_trailing_comma,
        })
    }
}
