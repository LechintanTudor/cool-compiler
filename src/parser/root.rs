use crate::parser::ast::{FnAst, RootAst};
use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_root(&mut self) -> anyhow::Result<RootAst> {
        let mut fns = Vec::<FnAst>::new();

        while !self.consume_if_eof() {
            fns.push(self.parse_fn()?);
        }

        Ok(RootAst { fns })
    }
}
