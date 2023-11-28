use crate::{ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;

define_index_newtype!(ExprId);

#[derive(Clone, Section, Debug)]
pub enum Expr {
    // Empty
}

impl Parser<'_> {
    pub fn parse_expr(&mut self) -> ParseResult<ExprId> {
        todo!()
    }

    pub fn parse_const_expr(&mut self) -> ParseResult<ExprId> {
        todo!()
    }
}
