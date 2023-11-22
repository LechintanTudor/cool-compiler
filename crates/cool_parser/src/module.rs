use crate::{Decl, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

impl Section for Module {
    #[inline]
    fn span(&self) -> Span {
        match self.decls.as_slice() {
            [] => Span::EMPTY,
            [first] => first.span(),
            [first, .., last] => first.span().to(last.span()),
        }
    }
}

impl Parser<'_> {
    pub fn parse_module(&mut self) -> ParseResult<Module> {
        let mut decls = Vec::<Decl>::new();

        while ![tk::close_brace, tk::eof].contains(&self.peek().kind) {
            decls.push(self.parse_decl()?);
        }

        Ok(Module { decls })
    }
}
