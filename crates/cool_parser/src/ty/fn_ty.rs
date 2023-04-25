use crate::{FnExternDecl, ParseResult, ParseTree, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnTyParamList {
    pub span: Span,
    pub params: Vec<Ty>,
    pub is_variadic: bool,
    pub has_trailing_comma: bool,
}

impl ParseTree for FnTyParamList {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnTy {
    pub span: Span,
    pub extern_decl: Option<FnExternDecl>,
    pub param_list: FnTyParamList,
    pub ret_ty: Option<Box<Ty>>,
}

impl ParseTree for FnTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    fn parse_fn_ty_param_list(&mut self) -> ParseResult<FnTyParamList> {
        let start_token = self.bump_expect(&tk::OPEN_PAREN)?;
        let mut params = Vec::<Ty>::new();

        let (end_span, is_variadic, has_trailing_comma) = match self.peek().kind {
            tk::CLOSE_PAREN => (self.bump().span, false, false),
            _ => loop {
                match self.peek().kind {
                    tk::DOT_DOT_DOT => {
                        self.bump_expect(&tk::DOT_DOT_DOT)?;
                        let end_token = self.bump_expect(&tk::CLOSE_PAREN)?;
                        break (end_token.span, true, false);
                    }
                    _ => {
                        params.push(self.parse_ty()?);
                    }
                }

                let next_token = self.bump();

                match next_token.kind {
                    tk::CLOSE_PAREN => {
                        break (next_token.span, false, false);
                    }
                    tk::COMMA => {
                        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                            break (end_token.span, false, true);
                        }
                    }
                    _ => self.error(next_token, &[tk::CLOSE_PAREN, tk::COMMA])?,
                }
            },
        };

        Ok(FnTyParamList {
            span: start_token.span.to(end_span),
            params,
            is_variadic,
            has_trailing_comma,
        })
    }

    pub fn parse_fn_ty(&mut self) -> ParseResult<FnTy> {
        let extern_decl = if self.peek().kind == tk::KW_EXTERN {
            Some(self.parse_fn_extern_decl()?)
        } else {
            None
        };

        let fn_kw = self.bump_expect(&tk::KW_FN)?;
        let param_list = self.parse_fn_ty_param_list()?;

        let return_ty = if self.bump_if_eq(tk::ARROW).is_some() {
            Some(Box::new(self.parse_ty()?))
        } else {
            None
        };

        let span = {
            let start_span = extern_decl
                .as_ref()
                .map(|decl| decl.span)
                .unwrap_or(fn_kw.span);

            let end_span = return_ty
                .as_ref()
                .map(|ty| ty.span())
                .unwrap_or(param_list.span);

            start_span.to(end_span)
        };

        Ok(FnTy {
            span,
            extern_decl,
            param_list,
            ret_ty: return_ty,
        })
    }
}
