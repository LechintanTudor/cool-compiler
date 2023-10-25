use crate::{IdentPath, IntLiteralExpr, ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub enum ArrayLen {
    Path(IdentPath),
    Int(IntLiteralExpr),
}

#[derive(Clone, Section, Debug)]
pub struct ArrayTy {
    pub span: Span,
    pub len: ArrayLen,
    pub elem_ty: Box<Ty>,
}

impl Parser<'_> {
    pub fn continue_parse_array_ty(&mut self, open_bracket: Token) -> ParseResult<ArrayTy> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);

        let len = self.parse_array_len()?;
        self.bump_expect(&tk::close_bracket)?;

        let elem_ty = self.parse_ty()?;

        Ok(ArrayTy {
            span: open_bracket.span.to(elem_ty.span()),
            len,
            elem_ty: Box::new(elem_ty),
        })
    }

    fn parse_array_len(&mut self) -> ParseResult<ArrayLen> {
        let len = if self.peek().kind.is_literal() {
            ArrayLen::Int(self.parse_int_literal_expr()?)
        } else {
            ArrayLen::Path(self.parse_ident_path()?)
        };

        Ok(len)
    }
}
