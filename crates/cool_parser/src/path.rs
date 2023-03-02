use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::symbols::sym;
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;
use smallvec::SmallVec;

pub type SymbolPathVec = SmallVec<[Ident; 2]>;

#[derive(Clone, Debug)]
pub struct SymbolPath {
    pub idents: SymbolPathVec,
}

impl ParseTree for SymbolPath {
    fn span(&self) -> Span {
        match (self.idents.first(), self.idents.last()) {
            (Some(first), Some(last)) => first.span_to(last),
            _ => Span::empty(),
        }
    }
}
impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_path(&mut self) -> ParseResult<SymbolPath> {
        match self.peek().kind {
            tk::KW_SELF => self.parse_self_path(),
            tk::KW_SUPER => self.parse_super_path(),
            tk::KW_CRATE => self.parse_crate_path(),
            _ => self.parse_ident_path(),
        }
    }

    fn parse_ident_path(&mut self) -> ParseResult<SymbolPath> {
        let mut idents = SymbolPathVec::new();
        idents.push(self.parse_ident()?);

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_ident()?);
        }

        Ok(SymbolPath { idents })
    }

    fn parse_self_path(&mut self) -> ParseResult<SymbolPath> {
        let start_token = self.bump_expect(&[tk::KW_SELF])?;

        let mut idents = SymbolPathVec::new();
        idents.push(Ident {
            symbol: sym::KW_SELF,
            span: start_token.span,
        });

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;

            let ident = self.parse_ident_including_super()?;
            idents.push(ident);

            if ident.symbol != sym::KW_SUPER {
                break;
            }
        }

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_ident()?);
        }

        Ok(SymbolPath { idents })
    }

    fn parse_super_path(&mut self) -> ParseResult<SymbolPath> {
        let start_token = self.bump_expect(&[tk::KW_SUPER])?;

        let mut idents = SymbolPathVec::new();
        idents.push(Ident {
            symbol: sym::KW_SUPER,
            span: start_token.span,
        });

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;

            let ident = self.parse_ident_including_super()?;
            idents.push(ident);

            if ident.symbol != sym::KW_SUPER {
                break;
            }
        }

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_ident()?);
        }

        Ok(SymbolPath { idents })
    }

    fn parse_crate_path(&mut self) -> ParseResult<SymbolPath> {
        let start_token = self.bump_expect(&[tk::KW_CRATE])?;

        let mut idents = SymbolPathVec::new();
        idents.push(Ident {
            symbol: sym::KW_CRATE,
            span: start_token.span,
        });

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_ident()?);
        }

        Ok(SymbolPath { idents })
    }
}
