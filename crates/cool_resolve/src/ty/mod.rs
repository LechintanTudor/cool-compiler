mod ty_config;
mod ty_consts;
mod ty_factory;
mod ty_id;
mod ty_kind;

pub use self::ty_config::*;
pub use self::ty_consts::*;
pub use self::ty_factory::*;
pub use self::ty_id::*;
pub use self::ty_kind::*;

use crate::{ItemId, ModuleId, ResolveContext, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn add_alias(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |_, _, _| tys::infer)
    }

    pub fn add_struct(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |context, item_id, _| {
            context.tys.insert(StructTy { item_id }.into())
        })
    }
}
