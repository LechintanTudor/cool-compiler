mod define_alias;
mod define_error;
mod define_struct;
mod define_ty;
mod resolve_binding;
mod resolve_error;
mod resolve_expr;
mod resolve_global;
mod resolve_local;

pub use self::define_alias::*;
pub use self::define_error::*;
pub use self::define_struct::*;
pub use self::define_ty::*;
pub use self::resolve_binding::*;
pub use self::resolve_error::*;
pub use self::resolve_expr::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
use crate::{tys, Binding, Frame, ItemKind, Module, Mutability, StructTy, TyKind};
use cool_arena::{Arena, SliceArena};
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;

#[derive(Debug)]
pub struct ResolveContext {
    paths: SliceArena<ItemId, Symbol>,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    tys: Arena<TyId, TyKind>,
    struct_tys: IdIndexedVec<StructId, Option<StructTy>>,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, TyId>,
}

impl Default for ResolveContext {
    fn default() -> Self {
        Self {
            paths: Default::default(),
            items: IdIndexedVec::new(ItemKind::Module(ModuleId::dummy())),
            modules: IdIndexedVec::new(Module {
                path: Default::default(),
                elems: Default::default(),
            }),
            tys: Default::default(),
            struct_tys: Default::default(),
            bindings: IdIndexedVec::new(Binding {
                mutability: Mutability::Immutable,
                ty_id: tys::INFERRED,
            }),
            frames: IdIndexedVec::new(Frame {
                parent_id: ModuleId::dummy().into(),
                bindings: Default::default(),
            }),
            exprs: Default::default(),
        }
    }
}
