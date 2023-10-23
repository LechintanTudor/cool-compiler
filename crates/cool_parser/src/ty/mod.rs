mod fn_ty;
mod paren_ty;
mod ptr_ty;
mod tuple_ty;

pub use self::fn_ty::*;
pub use self::paren_ty::*;
pub use self::ptr_ty::*;
pub use self::tuple_ty::*;

use crate::{IdentPath, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Ty {
    Fn(FnTy),
    Paren(ParenTy),
    Path(IdentPath),
    Ptr(PtrTy),
    Tuple(TupleTy),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty = match self.peek().kind {
            tk::open_paren => self.parse_paren_or_tuple_ty()?,
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
                elems,
                has_trailing_comma,
            }
            .into()
        };

        Ok(expr)
    }
}
