use crate::lexer::{Keyword, Separator};
use crate::parser::ast::{FnArgAst, FnArgListAst, FnAst};
use crate::parser::Parser;
use anyhow::{bail, Ok};

impl Parser<'_> {
    pub fn parse_fn(&mut self) -> anyhow::Result<FnAst> {
        if !self.next().is(Keyword::Fn) {
            bail!("missing 'fn' in function expression");
        }

        if !self.next().is(Separator::OpenParanthesis) {
            bail!("missing '(' in function expression");
        }

        let arg_list_ast = self.parse_fn_arg_list()?;

        if !self.next().is(Separator::ClosedParanthesis) {
            bail!("missing ')' in function expression");
        }

        if !self.next().is(Separator::OpenBrace) {
            bail!("missing '{{' in function expression");
        }

        if !self.next().is(Separator::ClosedBrace) {
            bail!("missing '}}' in function expression");
        }

        Ok(FnAst { args: arg_list_ast })
    }

    pub fn parse_fn_arg_list(&mut self) -> anyhow::Result<FnArgListAst> {
        let mut arg_list = Vec::<FnArgAst>::new();

        loop {
            if self.peek().is(Separator::ClosedParanthesis) {
                break;
            }

            arg_list.push(self.parse_fn_arg()?);

            if !self.consume_if_eq(Separator::Comma) {
                if !self.peek().is(Separator::ClosedParanthesis) {
                    bail!("mssing ')' at the end of function argument list");
                }
            }
        }

        Ok(FnArgListAst { args: arg_list })
    }

    pub fn parse_fn_arg(&mut self) -> anyhow::Result<FnArgAst> {
        let is_mutable = self.consume_if_eq(Keyword::Mut);

        let Some(ident_index) = self.next().as_ident_index() else {
            bail!("missing identifier in function argument");
        };

        if !self.next().is(Separator::Colon) {
            bail!("missing ':' in function argument");
        }

        let Some(type_ident_index) = self.next().as_ident_index() else {
            bail!("missing type indentifier in function argument");
        };

        Ok(FnArgAst {
            is_mutable,
            ident_index,
            type_ident_index,
        })
    }
}
