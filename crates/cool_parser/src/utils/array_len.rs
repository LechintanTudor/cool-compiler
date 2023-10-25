use crate::{IdentPath, IntLiteralExpr, ParseResult, Parser};
use cool_derive::Section;

#[derive(Clone, Section, Debug)]
pub enum ArrayLen {
    Path(IdentPath),
    Int(IntLiteralExpr),
}

impl Parser<'_> {
    pub fn parse_array_len(&mut self) -> ParseResult<ArrayLen> {
        let len = if self.peek().kind.is_literal() {
            ArrayLen::Int(self.parse_int_literal_expr()?)
        } else {
            ArrayLen::Path(self.parse_ident_path()?)
        };

        Ok(len)
    }
}
