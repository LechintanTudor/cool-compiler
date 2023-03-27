use crate::expr::BlockExpr;
use crate::{ExternFnItem, FnPrototype, Item, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnItem {
    pub prototype: FnPrototype,
    pub body: BlockExpr,
}

impl ParseTree for FnItem {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span_to(&self.body)
    }
}

impl Parser<'_> {
    pub fn parse_fn_or_extern_fn_item(&mut self) -> ParseResult<Item> {
        let prototype = self.parse_fn_prototype()?;

        let body = if prototype.extern_decl.is_some() {
            if self.peek().kind == tk::OPEN_BRACE {
                Some(self.parse_block_expr()?)
            } else {
                None
            }
        } else {
            Some(self.parse_block_expr()?)
        };

        let item: Item = match body {
            Some(body) => FnItem { prototype, body }.into(),
            None => ExternFnItem { prototype }.into(),
        };

        Ok(item)
    }
}
