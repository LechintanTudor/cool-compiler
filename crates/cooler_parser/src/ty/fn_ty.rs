use crate::{parse_error, ParseResult, Parser, TyId};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::{tk, Symbol};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnTy {
    pub span: Span,
    pub abi: Option<Option<Symbol>>,
    pub param_tys: SmallVec<TyId, 4>,
    pub has_trailing_comma: bool,
    pub is_variadic: bool,
    pub return_ty: Option<TyId>,
}

impl Parser<'_> {
    pub fn parse_fn_ty(&mut self) -> ParseResult<TyId> {
        let abi_decl = (self.peek().kind == tk::kw_extern)
            .then(|| self.parse_fn_abi_decl())
            .transpose()?;

        let fn_token = self.bump_expect(&tk::kw_fn)?;
        self.bump_expect(&tk::open_paren)?;

        let mut param_tys = SmallVec::new();

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
                    println!("{}", next_token.kind);

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
            .map(|abi| abi.span)
            .unwrap_or(fn_token.span);

        let end_span = return_ty
            .map(|ty| self[ty].span())
            .unwrap_or(close_paren.span);

        Ok(self.add_ty(FnTy {
            span: start_span.to(end_span),
            abi: abi_decl.map(|decl| decl.abi),
            param_tys,
            has_trailing_comma,
            is_variadic,
            return_ty,
        }))
    }
}
