mod error;
mod expr;
mod item;
mod scope;
mod ty;

pub use self::error::*;
pub use self::expr::*;
pub use self::item::*;
pub use self::scope::*;
pub use self::ty::*;

use ahash::AHashMap;
use cool_collections::{Arena, VecMap};
use cool_lexer::Symbol;

#[derive(Debug)]
pub struct ResolveContext<'a> {
    paths: Arena<'a, ItemId, [Symbol]>,
    items: AHashMap<ItemId, ItemKind>,
    modules: VecMap<ModuleId, ModuleItem>,
    ty_config: TyConfig,
    tys: Arena<'a, TyId, TyKind>,
    ty_defs: AHashMap<TyId, TyDef>,
    frames: VecMap<FrameId, Frame>,
    bindings: VecMap<BindingId, Binding>,
    exprs: VecMap<ExprId, Expr>,
}

impl<'a> ResolveContext<'a> {
    pub fn new_leak(ty_config: TyConfig) -> Self {
        let mut ctx = Self {
            paths: Arena::new_leak(),
            items: Default::default(),
            modules: Default::default(),
            ty_config,
            tys: Arena::new_leak(),
            ty_defs: Default::default(),
            frames: Default::default(),
            bindings: Default::default(),
            exprs: Default::default(),
        };

        ctx.init_builtins();
        ctx
    }
}
