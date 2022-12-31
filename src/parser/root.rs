use crate::parser::{FnAst, Parser};

#[derive(Clone, Debug)]
pub struct RootAst {
    pub fns: Vec<FnAst>,
}

impl Parser<'_> {
    pub fn parse_root(&mut self) -> anyhow::Result<RootAst> {
        let mut fns = Vec::<FnAst>::new();

        while !self.consume_if_eof() {
            fns.push(self.parse_fn()?);
        }

        Ok(RootAst { fns })
    }
}
