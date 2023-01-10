use crate::lexer::{Keyword, Separator, Token};
use crate::parser::{Parser, TyAst};

#[derive(Clone, Debug)]
pub struct FnArgAst {
    pub is_mutable: bool,
    pub ident_index: u32,
    pub ty: TyAst,
}

#[derive(Clone, Debug)]
pub struct FnArgListAst {
    pub args: Vec<FnArgAst>,
}

#[derive(Clone, Debug)]
pub struct FnAst {
    pub args: FnArgListAst,
}

impl Parser<'_> {
    pub fn parse_fn(&mut self) -> anyhow::Result<FnAst> {
        if !self.next().is(Keyword::Fn) {
            panic!("missing 'fn' in function expression");
        }

        let arg_list_ast = self.parse_fn_arg_list()?;

        if !self.next().is(Separator::OpenBrace) {
            panic!("missing '{{' in function expression");
        }

        if !self.next().is(Separator::ClosedBrace) {
            panic!("missing '}}' in function expression");
        }

        Ok(FnAst { args: arg_list_ast })
    }

    pub fn parse_fn_arg_list(&mut self) -> anyhow::Result<FnArgListAst> {
        if !self.consume_if_eq(Separator::OpenParen) {
            panic!("missing '(' in function argument list");
        }

        if self.consume_if_eq(Separator::ClosedParen) {
            return Ok(FnArgListAst { args: Vec::new() });
        }

        let mut arg_list = Vec::<FnArgAst>::new();

        loop {
            arg_list.push(self.parse_fn_arg()?);

            match self.next() {
                Token::Separator(Separator::Comma) => {
                    if self.consume_if_eq(Separator::ClosedParen) {
                        break;
                    }
                }
                Token::Separator(Separator::ClosedParen) => break,
                _ => panic!("unexpected token while parsing argument list"),
            }
        }

        Ok(FnArgListAst { args: arg_list })
    }

    pub fn parse_fn_arg(&mut self) -> anyhow::Result<FnArgAst> {
        let is_mutable = self.consume_if_eq(Keyword::Mut);

        let Some(ident_index) = self.next().as_ident_index() else {
            panic!("missing identifier in function argument");
        };

        if !self.next().is(Separator::Colon) {
            panic!("missing ':' in function argument");
        }

        let Ok(ty) = self.parse_ty() else {
            panic!("failed to parse type in function argument");
        };

        Ok(FnArgAst {
            is_mutable,
            ident_index,
            ty,
        })
    }
}
