mod decl;
mod expr;
mod ident;
mod item;
mod ty;

pub use self::decl::*;
pub use self::expr::*;
pub use self::ident::*;
pub use self::item::*;
pub use self::ty::*;

use cool_collections::VecMap;
use cool_lexer::TokenStream;

#[derive(Clone, Debug)]
pub struct ParserData {
    pub decls: VecMap<DeclId, Decl>,
    pub items: VecMap<ItemId, InlineItem>,
    pub modules: VecMap<ModuleId, Module>,
    pub imports: VecMap<ImportId, Import>,
    pub structs: VecMap<StructId, Struct>,
    pub tys: VecMap<TyId, Ty>,
    pub exprs: VecMap<ExprId, Expr>,
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
}
