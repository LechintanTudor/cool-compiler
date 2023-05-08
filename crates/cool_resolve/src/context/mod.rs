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
use crate::{tys, Binding, Frame, ItemKind, Module, Mutability, PrimitiveTyProps, TyContext};
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;

#[derive(Debug)]
pub struct ResolveContext {
    paths: SliceArena<ItemId, Symbol>,
    items: IdIndexedVec<ItemId, ItemKind>,
    modules: IdIndexedVec<ModuleId, Module>,
    tys: TyContext,
    bindings: IdIndexedVec<BindingId, Binding>,
    frames: IdIndexedVec<FrameId, Frame>,
    exprs: IdIndexedVec<ExprId, ResolveExpr>,
}

impl ResolveContext {
    pub(crate) fn empty(primitives: PrimitiveTyProps) -> Self {
        Self {
            paths: Default::default(),
            items: IdIndexedVec::new(ItemKind::Module(ModuleId::dummy())),
            modules: IdIndexedVec::new(Module {
                path: Default::default(),
                elems: Default::default(),
            }),
            tys: TyContext::new(primitives),
            bindings: IdIndexedVec::new(Binding {
                symbol: Symbol::dummy(),
                mutability: Mutability::Immutable,
                ty_id: tys::INFERRED,
            }),
            frames: IdIndexedVec::new(Frame {
                parent: ModuleId::dummy().into(),
                bindings: Default::default(),
            }),
            exprs: Default::default(),
        }
    }
}
