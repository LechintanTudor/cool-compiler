use crate::{parse_error, ExprId, FnAbiDecl, ParseResult, Parser, Pattern, TyId};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};
use derive_more::From;

define_index_newtype!(FnProtoId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum FnExprOrFnProto {
    Expr(ExprId),
    Proto(FnProtoId),
}

#[derive(Clone, Section, Debug)]
pub struct FnExpr {
    pub span: Span,
    pub proto: FnProtoId,
    pub body: ExprId,
}

#[derive(Clone, Section, Debug)]
pub struct FnProto {
    pub span: Span,
    pub abi_decl: Option<FnAbiDecl>,
    pub params: Vec<FnParam>,
    pub is_variadic: bool,
    pub has_trailing_comma: bool,
    pub return_ty: Option<TyId>,
}

#[derive(Clone, Debug)]
pub struct FnParam {
    pub pattern: Pattern,
    pub ty: Option<TyId>,
}

impl Parser<'_> {
    pub fn parse_fn_expr_or_fn_proto(&mut self) -> ParseResult<FnExprOrFnProto> {
        let proto = self.parse_fn_proto()?;

        if self[proto].abi_decl.is_some() && self.peek().kind != tk::open_brace {
            return Ok(proto.into());
        }

        let body = self.parse_block_expr()?;

        let proto_span = self[proto].span;
        let body_span = self[body].span();

        let expr_id = self.add_expr(FnExpr {
            span: proto_span.to(body_span),
            proto,
            body,
        });

        Ok(expr_id.into())
    }

    pub fn parse_fn_expr(&mut self) -> ParseResult<ExprId> {
        let proto = self.parse_fn_proto()?;
        let body = self.parse_block_expr()?;

        let proto_span = self[proto].span;
        let body_span = self[body].span();

        Ok(self.add_expr(FnExpr {
            span: proto_span.to(body_span),
            proto,
            body,
        }))
    }

    fn parse_fn_proto(&mut self) -> ParseResult<FnProtoId> {
        let abi_decl = (self.peek().kind == tk::kw_extern)
            .then(|| self.parse_fn_abi_decl())
            .transpose()?;

        let fn_token = self.bump_expect(&tk::kw_fn)?;
        self.bump_expect(&tk::open_paren)?;

        let mut params = Vec::<FnParam>::new();

        let (close_paren, is_variadic, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false, false)
            } else {
                loop {
                    if self.bump_if_eq(tk::dot_dot_dot).is_some() {
                        break (self.bump_expect(&tk::close_paren)?, true, false);
                    }

                    params.push(self.parse_fn_param()?);

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
                        _ => return parse_error(next_token, &[tk::close_paren, tk::comma]),
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
            .map(|ty| self[ty].span())
            .unwrap_or(close_paren.span);

        Ok(self.add_fn_proto(FnProto {
            span: start_span.to(end_span),
            abi_decl,
            params,
            is_variadic,
            has_trailing_comma,
            return_ty,
        }))
    }

    fn parse_fn_param(&mut self) -> ParseResult<FnParam> {
        let pattern = self.parse_pattern()?;

        let ty = (self.bump_if_eq(tk::colon).is_some())
            .then(|| self.parse_ty())
            .transpose()?;

        Ok(FnParam { pattern, ty })
    }
}
