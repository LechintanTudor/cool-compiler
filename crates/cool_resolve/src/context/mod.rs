mod define_error;
mod resolve_alias;
mod resolve_binding;
mod resolve_error;
mod resolve_expr;
mod resolve_global;
mod resolve_local;
mod resolve_struct;
mod resolve_ty;

pub use self::define_error::*;
pub use self::resolve_alias::*;
pub use self::resolve_binding::*;
pub use self::resolve_error::*;
pub use self::resolve_expr::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
pub use self::resolve_struct::*;
pub use self::resolve_ty::*;
use crate::{Binding, Frame, ItemKind, Module, PrimitiveTys, TyContext};
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;

#[derive(Debug)]
pub struct ResolveContext {
    paths: SliceArena<'static, ItemId, Symbol>,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    tys: TyContext,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, ResolveExpr>,
}

impl ResolveContext {
    pub(crate) fn empty(primitives: PrimitiveTys) -> Self {
        Self {
            paths: SliceArena::new_leak(),
            items: Default::default(),
            modules: Default::default(),
            tys: TyContext::new(primitives),
            bindings: Default::default(),
            frames: Default::default(),
            exprs: Default::default(),
        }
    }
}
