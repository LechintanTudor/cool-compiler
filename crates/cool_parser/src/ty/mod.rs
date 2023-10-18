mod fn_ty;

pub use self::fn_ty::*;

use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Ty {
    Fn(FnTy),
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let ty = match self.peek().kind {
            tk::kw_extern | tk::kw_fn => self.parse_fn_ty()?.into(),
            _ => return self.peek_error(&[tk::kw_extern, tk::kw_fn]),
        };

        Ok(ty)
    }
}
