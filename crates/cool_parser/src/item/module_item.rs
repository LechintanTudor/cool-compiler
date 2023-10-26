use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ModuleItem {
    pub span: Span,
}

impl Parser<'_> {
    #[inline]
    pub fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        Ok(ModuleItem {
            span: self.bump_expect(&tk::kw_module)?.span,
        })
    }
}
