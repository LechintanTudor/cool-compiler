mod resolve_error;
mod resolve_global;
mod resolve_local;
mod resolve_types;

pub use self::resolve_error::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
pub use self::resolve_types::*;
use crate::expr_ty::ExprId;
use crate::ty::{tys, TyId, TyKind};
use cool_arena::{Arena, SliceArena};
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;

#[derive(Debug)]
pub struct ResolveTable {
    paths: SliceArena<ItemId, Symbol>,
    tys: Arena<TyId, TyKind>,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, TyId>,
}

impl ResolveTable {
    pub fn with_builtins() -> Self {
        Self::default()
    }

    pub fn insert_builtin_item(&mut self, _item_id: ItemId, _symbol: Symbol) {
        todo!()
    }
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
