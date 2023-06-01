use crate::expr::BlockExpr;
use crate::{AbstractFn, ExternFnItem, FnPrototype, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct FnExpr {
    pub prototype: Box<FnPrototype>,
    pub body: Box<BlockExpr>,
}

impl Section for FnExpr {
    #[inline]
    fn span(&self) -> Span {
        self.prototype.span().to(self.body.span())
    }
}

impl Parser<'_> {
    pub fn parse_fn_or_extern_fn_item(&mut self) -> ParseResult<AbstractFn> {
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

        let abstract_fn: AbstractFn = match body {
            Some(body) => {
                FnExpr {
                    prototype: Box::new(prototype),
                    body: Box::new(body),
                }
                .into()
            }
            None => ExternFnItem { prototype }.into(),
        };

        Ok(abstract_fn)
    }
}
