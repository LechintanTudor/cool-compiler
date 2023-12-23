mod decl;
mod import;

pub use self::decl::*;
pub use self::import::*;

use crate::{
    Expr, ExprId, FnProto, FnProtoId, Item, ItemId, Module, ModuleId, ParseResult, Parser, Stmt,
    StmtId, Struct, StructId, Ty, TyId,
};
use cool_collections::VecMap;
use paste::paste;
use std::ops::Index;

macro_rules! define_file {
    { $($field:ident: $Key:ty => $Value:ty,)* } => {
        paste! {
            #[derive(Clone, Default, Debug)]
            pub struct File {
                $(pub [<$field s>]: VecMap<$Key, $Value>,)*
            }

            impl Parser<'_> {
                $(
                    pub fn [<add_ $field>]<T>(&mut self, [<$field _id>]: T) -> $Key
                    where
                        T: Into<$Value>,
                    {
                        self.file.[<$field s>].push([<$field _id>].into())
                    }
                )*
            }

            $(
                impl Index<$Key> for Parser<'_> {
                    type Output = $Value;

                    #[inline]
                    fn index(&self, [<$field _id>]: $Key) -> &Self::Output {
                        &self.file.[<$field s>][[<$field _id>]]
                    }
                }
            )*
        }
    };
}

define_file! {
    module: ModuleId => Module,
    decl: DeclId => Decl,
    item: ItemId => Item,
    import: ImportId => Import,
    struct: StructId => Struct,
    fn_proto: FnProtoId => FnProto,
    ty: TyId => Ty,
    stmt: StmtId => Stmt,
    expr: ExprId => Expr,
}

#[inline]
pub fn parse_file(source: &str) -> ParseResult<File> {
    let mut parser = Parser::new(source);
    parser.parse_file_module()?;
    Ok(parser.file)
}
