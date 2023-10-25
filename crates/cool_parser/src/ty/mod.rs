mod array_ty;
mod fn_ty;
mod many_ptr_ty;
mod paren_ty;
mod ptr_ty;
mod slice_ty;
mod tuple_ty;

pub use self::array_ty::*;
pub use self::fn_ty::*;
pub use self::many_ptr_ty::*;
pub use self::paren_ty::*;
pub use self::ptr_ty::*;
pub use self::slice_ty::*;
pub use self::tuple_ty::*;

use crate::{IdentPath, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Ty {
    Array(ArrayTy),
    Fn(FnTy),
    ManyPtr(ManyPtrTy),
    Paren(ParenTy),
    Path(IdentPath),
    Ptr(PtrTy),
    Slice(SliceTy),
    Tuple(TupleTy),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty = match self.peek().kind {
            tk::open_paren => self.parse_paren_or_tuple_ty()?,
            tk::open_bracket => {
                let open_bracket = self.bump();

                match self.peek().kind {
                    tk::close_bracket => self.continue_parse_slice_ty(open_bracket)?.into(),
                    tk::star => self.continue_parse_many_ptr_ty(open_bracket)?.into(),
                    _ => self.continue_parse_array_ty(open_bracket)?.into(),
                }
            }
            tk::star => self.parse_ptr_ty()?.into(),
            tk::kw_extern | tk::kw_fn => self.parse_fn_ty()?.into(),
            TokenKind::Ident(_) => self.parse_ident_path()?.into(),
            _ => {
                return self.peek_error(&[
                    tk::open_paren,
                    tk::star,
                    tk::kw_extern,
                    tk::kw_fn,
                    tk::identifier,
                ]);
            }
        };

        Ok(ty)
    }

    fn parse_paren_or_tuple_ty(&mut self) -> ParseResult<Ty> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut elems = Vec::<Ty>::new();

        let (close_paren, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false)
            } else {
                loop {
                    elems.push(self.parse_ty()?);
                    let token = self.bump();

                    match token.kind {
                        tk::comma => {
                            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                                break (close_paren, true);
                            }
                        }
                        tk::close_paren => break (token, false),
                        _ => return self.error(token, &[tk::comma, tk::close_paren]),
                    }
                }
            };

        let expr = if elems.len() == 1 && !has_trailing_comma {
            ParenTy {
                span: open_paren.span.to(close_paren.span),
                ty: Box::new(elems.pop().unwrap()),
            }
            .into()
        } else {
            TupleTy {
                span: open_paren.span.to(close_paren.span),
                elem_tys: elems,
                has_trailing_comma,
            }
            .into()
        };

        Ok(expr)
    }
}
