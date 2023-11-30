mod decl;
mod error;
mod expr;
mod item;
mod stmt;
mod ty;
mod utils;

pub use self::decl::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;

use cool_collections::VecMap;
use cool_lexer::{Token, TokenKind, TokenStream};
use paste::paste;
use std::ops::Index;
use std::slice;

macro_rules! define_parser_data {
    { $($field:ident: $Key:ty => $Value:ty,)* } => {
        paste! {
            #[derive(Clone, Default, Debug)]
            pub struct ParserData {
                $(pub [<$field s>]: VecMap<$Key, $Value>,)*
            }

            impl Parser<'_> {
                $(
                    pub fn [<add_ $field>]<T>(&mut self, [<$field _id>]: T) -> $Key
                    where
                        T: Into<$Value>,
                    {
                        self.data.[<$field s>].push([<$field _id>].into())
                    }
                )*
            }

            $(
                impl Index<$Key> for Parser<'_> {
                    type Output = $Value;

                    #[inline]
                    fn index(&self, [<$field _id>]: $Key) -> &Self::Output {
                        &self.data.[<$field s>][[<$field _id>]]
                    }
                }
            )*
        }
    };
}

define_parser_data! {
    decl: DeclId => Decl,
    item: ItemId => Item,
    module: ModuleId => Module,
    import: ImportId => Import,
    struct: StructId => Struct,
    ty: TyId => Ty,
    expr: ExprId => Expr,
    stmt: StmtId => Stmt,
}

#[derive(Debug)]
pub struct Parser<'a> {
    data: &'a mut ParserData,
    tokens: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    #[inline]
    pub fn new(data: &'a mut ParserData, source: &'a str) -> Self {
        Self {
            data,
            tokens: TokenStream::new(source),
        }
    }

    #[inline]
    pub fn bump(&mut self) -> Token {
        self.tokens.next_lang()
    }

    #[inline]
    pub fn bump_any(&mut self) -> Token {
        self.tokens.next_any()
    }

    #[inline]
    #[must_use]
    pub fn peek(&mut self) -> Token {
        self.tokens.peek_lang()
    }

    #[inline]
    #[must_use]
    pub fn peek_any(&mut self) -> Token {
        self.tokens.peek_any()
    }

    pub fn bump_if_eq(&mut self, expected_token: TokenKind) -> Option<Token> {
        if self.peek().kind != expected_token {
            return None;
        }

        Some(self.bump())
    }

    pub fn bump_any_if_eq(&mut self, expected_token: TokenKind) -> Option<Token> {
        if self.peek_any().kind != expected_token {
            return None;
        }

        Some(self.tokens.next_any())
    }

    pub fn bump_expect(&mut self, expected_token: &'static TokenKind) -> ParseResult<Token> {
        let token = self.bump();

        if &token.kind != expected_token {
            return Err(ParseError {
                found: token,
                expected: slice::from_ref(expected_token),
            });
        }

        Ok(token)
    }

    pub fn peek_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError {
            found: self.peek(),
            expected,
        })
    }

    pub fn peek_any_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError {
            found: self.peek_any(),
            expected,
        })
    }
}
