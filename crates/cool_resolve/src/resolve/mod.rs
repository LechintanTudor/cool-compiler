mod resolve_binding;
mod resolve_error;
mod resolve_expr;
mod resolve_global;
mod resolve_item;
mod resolve_local;
mod resolve_scope;
mod resolve_ty;

pub use self::resolve_binding::*;
pub use self::resolve_error::*;
pub use self::resolve_expr::*;
pub use self::resolve_global::*;
pub use self::resolve_item::*;
pub use self::resolve_local::*;
pub use self::resolve_scope::*;
pub use self::resolve_ty::*;
use crate::ty::{tys, TyId};
use crate::TyTable;
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;
use std::ops;

#[derive(Debug)]
pub struct ResolveTable {
    paths: SliceArena<ItemId, Symbol>,
    tys: TyTable,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, TyId>,
}

impl Default for ResolveTable {
    fn default() -> Self {
        Self {
            paths: Default::default(),
            tys: Default::default(),
            items: IdIndexedVec::new(ItemKind::Module(ModuleId::dummy())),
            modules: IdIndexedVec::new(Module {
                path: Default::default(),
                elems: Default::default(),
            }),
            bindings: IdIndexedVec::new(Binding {
                mutability: Mutability::Immutable,
                ty_id: tys::INFERRED,
            }),
            frames: IdIndexedVec::new(Frame {
                parent_id: ScopeId::Module(ModuleId::dummy()),
                bindings: Default::default(),
            }),
            exprs: Default::default(),
        }
    }
}

impl ops::Index<ItemId> for ResolveTable {
    type Output = ItemKind;

    #[inline]
    fn index(&self, item_id: ItemId) -> &Self::Output {
        &self.items[item_id]
    }
}
