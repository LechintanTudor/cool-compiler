use crate::{BlockExpr, FnAbiDecl, ParseResult, Parser, Pattern, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum FnOrExternFnExpr {
    Fn(FnExpr),
    ExternFn(ExternFnExpr),
}

#[derive(Clone, Debug)]
pub struct FnExpr {
    pub prototype: FnExprPrototype,
    pub body: Box<BlockExpr>,
}

impl Section for FnExpr {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span.to(self.body.span)
    }
}

#[derive(Clone, Debug)]
pub struct ExternFnExpr {
    pub prototype: FnExprPrototype,
}

impl Section for ExternFnExpr {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span
    }
}

#[derive(Clone, Section, Debug)]
pub struct FnExprPrototype {
    pub span: Span,
    pub abi_decl: Option<FnAbiDecl>,
    pub params: Vec<FnExprParam>,
    pub is_variadic: bool,
    pub has_trailing_comma: bool,
    pub return_ty: Option<Box<Ty>>,
}

#[derive(Clone, Debug)]
pub struct FnExprParam {
    pub pattern: Pattern,
    pub ty: Option<Ty>,
}

impl Section for FnExprParam {
    #[inline]
    fn span(&self) -> Span {
        match self.ty.as_ref() {
            Some(ty) => self.pattern.span.to(ty.span()),
            None => self.pattern.span,
        }
    }
}

impl Parser<'_> {
    pub fn parse_fn_or_extern_fn_expr(&mut self) -> ParseResult<FnOrExternFnExpr> {
        let prototype = self.parse_fn_expr_prototype()?;

        let body = (self.peek().kind == tk::open_brace)
            .then(|| self.parse_block_expr())
            .transpose()?;

        let expr: FnOrExternFnExpr = match body {
            Some(body) => {
                FnExpr {
                    prototype,
                    body: Box::new(body),
                }
                .into()
            }
            None => ExternFnExpr { prototype }.into(),
        };

        Ok(expr)
    }

    pub fn parse_fn_expr(&mut self) -> ParseResult<FnExpr> {
        Ok(FnExpr {
            prototype: self.parse_fn_expr_prototype()?,
            body: Box::new(self.parse_block_expr()?),
        })
    }

    fn parse_fn_expr_prototype(&mut self) -> ParseResult<FnExprPrototype> {
        let abi_decl = (self.peek().kind == tk::kw_extern)
            .then(|| self.parse_fn_abi_decl())
            .transpose()?;

        let fn_token = self.bump_expect(&tk::kw_fn)?;
        self.bump_expect(&tk::open_paren)?;

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

        let return_ty = self
            .bump_if_eq(tk::arrow)
            .map(|_| self.parse_ty())
            .transpose()?;

        let start_span = abi_decl
            .as_ref()
            .map(|decl| decl.span)
            .unwrap_or(fn_token.span);

        let end_span = return_ty
            .as_ref()
            .map(|ty| ty.span())
            .unwrap_or(close_paren.span);

        Ok(FnExprPrototype {
            span: start_span.to(end_span),
            abi_decl,
            params,
            is_variadic,
            has_trailing_comma,
            return_ty: return_ty.map(Box::new),
        })
    }

    fn parse_fn_expr_param(&mut self) -> ParseResult<FnExprParam> {
        let pattern = self.parse_pattern()?;

        let ty = self
            .bump_if_eq(tk::colon)
            .map(|_| self.parse_ty())
            .transpose()?;

        Ok(FnExprParam { pattern, ty })
    }
}
