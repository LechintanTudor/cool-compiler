use crate::lexer::{Separator, TokenKind};
use crate::parser::Parser;

#[derive(Clone, Debug)]
pub enum TyAst {
    Ident { index: u32 },
    Tuple { tys: Vec<TyAst> },
    Slice { ty: Box<TyAst> },
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> anyhow::Result<TyAst> {
        if self.peek().is(Separator::OpenParen) {
            self.parse_tuple()
        } else if self.peek().is(Separator::OpenBracket) {
            self.parse_slice()
        } else if let Some(ident_index) = self.next().as_ident_index() {
            Ok(TyAst::Ident { index: ident_index })
        } else {
            panic!("unexpected token while parsing type");
        }
    }

    fn parse_tuple(&mut self) -> anyhow::Result<TyAst> {
        if !self.next().is(Separator::OpenParen) {
            panic!("missing '(' in tuple type");
        }

        if self.consume_if_eq(Separator::ClosedParen) {
            return Ok(TyAst::Tuple { tys: Vec::new() });
        }

        let mut tys = Vec::<TyAst>::new();

        loop {
            tys.push(self.parse_ty()?);

            match self.next() {
                TokenKind::Separator(Separator::Comma) => {
                    if self.consume_if_eq(Separator::ClosedParen) {
                        break;
                    }
                }
                TokenKind::Separator(Separator::ClosedParen) => {
                    if tys.len() == 1 {
                        panic!("missing ',' in tuple of length 1");
                    }

                    break;
                }
                _ => panic!("unexpected token while parsing tuple type"),
            }
        }

        Ok(TyAst::Tuple { tys })
    }

    fn parse_slice(&mut self) -> anyhow::Result<TyAst> {
        if !self.next().is(Separator::OpenBracket) {
            panic!("missing '[' in slice type");
        }

        if !self.next().is(Separator::ClosedBracket) {
            panic!("mssing '] in slice type");
        }

        let ty = self.parse_ty()?;
        Ok(TyAst::Slice { ty: Box::new(ty) })
    }
}
