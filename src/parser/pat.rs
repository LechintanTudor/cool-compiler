use crate::lexer::{Keyword, Separator, TokenKind};
use crate::parser::Parser;

#[derive(Clone, Debug)]
pub enum PatAst {
    Wildcard,
    Ident(IdentAst),
    Tuple(TuplePatAst),
    Array(ArrayPatAst),
}

#[derive(Clone, Debug)]
pub struct IdentAst {
    pub mutable: bool,
    pub index: u32,
}

#[derive(Clone, Debug)]
pub struct TuplePatAst {
    pub elems: Vec<PatAst>,
}

#[derive(Clone, Debug)]
pub struct ArrayPatAst {
    pub elems: Vec<PatAst>,
}

impl Parser<'_> {
    pub fn parse_pat(&mut self) -> anyhow::Result<PatAst> {
        if self.consume_if_eq(TokenKind::Underscore) {
            Ok(PatAst::Wildcard)
        } else if self.peek_eq(Keyword::Mut) {
            self.parse_ident_pat().map(PatAst::Ident)
        } else if matches!(self.peek(), TokenKind::Ident { .. }) {
            self.parse_ident_pat().map(PatAst::Ident)
        } else if self.peek_eq(Separator::OpenParen) {
            self.parse_tuple_pat().map(PatAst::Tuple)
        } else if self.peek_eq(Separator::OpenBracket) {
            self.parse_array_pat().map(PatAst::Array)
        } else {
            panic!("unexpected token while parsing pattern");
        }
    }

    fn parse_ident_pat(&mut self) -> anyhow::Result<IdentAst> {
        let mutable = self.consume_if_eq(Keyword::Mut);

        let Some(index) = self.next().as_ident_index() else {
            panic!("unexpected token while parsing identifier pattern");
        };

        Ok(IdentAst { mutable, index })
    }

    fn parse_tuple_pat(&mut self) -> anyhow::Result<TuplePatAst> {
        if !self.consume_if_eq(Separator::OpenParen) {
            panic!("missing '(' while parsing tuple pattern");
        }

        if self.consume_if_eq(Separator::ClosedParen) {
            return Ok(TuplePatAst { elems: Vec::new() });
        }

        let mut elems = Vec::<PatAst>::new();

        loop {
            elems.push(self.parse_pat()?);

            if self.consume_if_eq(Separator::Comma) {
                if self.consume_if_eq(Separator::ClosedParen) {
                    break;
                }
            } else if self.consume_if_eq(Separator::ClosedParen) {
                break;
            } else {
                panic!("unexpected token while parsing tuple pattern");
            }
        }

        Ok(TuplePatAst { elems })
    }

    fn parse_array_pat(&mut self) -> anyhow::Result<ArrayPatAst> {
        if !self.consume_if_eq(Separator::OpenBracket) {
            panic!("missing '[' while parsing array pattern");
        }

        let mut elems = Vec::<PatAst>::new();

        loop {
            elems.push(self.parse_pat()?);

            if self.consume_if_eq(Separator::Comma) {
                if self.consume_if_eq(Separator::ClosedBracket) {
                    break;
                }
            } else if self.consume_if_eq(Separator::ClosedBracket) {
                break;
            } else {
                panic!("unexpected token while parsing array pattern");
            }
        }

        Ok(ArrayPatAst { elems })
    }
}
