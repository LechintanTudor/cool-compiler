mod fn_ty;

pub use self::fn_ty::*;

use crate::{IdentPath, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Ty {
    Fn(FnTy),
    Path(IdentPath),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty = match self.peek().kind {
            tk::kw_extern | tk::kw_fn => self.parse_fn_ty()?.into(),
            TokenKind::Ident(_) => self.parse_ident_path()?.into(),
            _ => return self.peek_error(&[tk::kw_extern, tk::kw_fn, tk::identifier]),
        };

        Ok(ty)
    }
}
