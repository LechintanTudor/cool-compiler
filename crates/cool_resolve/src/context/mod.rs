mod define_alias;
mod define_enum;
mod define_error;
mod define_struct;
mod resolve_binding;
mod resolve_error;
mod resolve_expr;
mod resolve_global;
mod resolve_local;
mod resolve_ty;

pub use self::define_alias::*;
pub use self::define_enum::*;
pub use self::define_error::*;
pub use self::define_struct::*;
pub use self::resolve_binding::*;
pub use self::resolve_error::*;
pub use self::resolve_expr::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
pub use self::resolve_ty::*;
use crate::{Binding, Frame, ItemKind, Module, PrimitiveTyData, TyContext};
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::{sym, Symbol};

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
    pub fn new(primitives: PrimitiveTyData) -> Self {
        let mut resolve = Self::empty(primitives);
        resolve.insert_root_module(sym::EMPTY).unwrap();
        resolve.init_primitive_item_tys();
        resolve
    }

    fn empty(primitives: PrimitiveTyData) -> Self {
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

    fn init_primitive_item_tys(&mut self) {
        // Non-number primitives
        self.insert_primitive_item_ty(sym::BOOL, self.ty_consts().bool);
        self.insert_primitive_item_ty(sym::CHAR, self.ty_consts().char);

        // Signed integers
        self.insert_primitive_item_ty(sym::I8, self.ty_consts().i8);
        self.insert_primitive_item_ty(sym::I16, self.ty_consts().i16);
        self.insert_primitive_item_ty(sym::I32, self.ty_consts().i32);
        self.insert_primitive_item_ty(sym::I64, self.ty_consts().i64);
        self.insert_primitive_item_ty(sym::I128, self.ty_consts().i128);
        self.insert_primitive_item_ty(sym::ISIZE, self.ty_consts().isize);

        // Unsigned integers
        self.insert_primitive_item_ty(sym::U8, self.ty_consts().u8);
        self.insert_primitive_item_ty(sym::U16, self.ty_consts().u16);
        self.insert_primitive_item_ty(sym::U32, self.ty_consts().u32);
        self.insert_primitive_item_ty(sym::U64, self.ty_consts().u64);
        self.insert_primitive_item_ty(sym::U128, self.ty_consts().u128);
        self.insert_primitive_item_ty(sym::USIZE, self.ty_consts().usize);

        // Floats
        self.insert_primitive_item_ty(sym::F32, self.ty_consts().f32);
        self.insert_primitive_item_ty(sym::F64, self.ty_consts().f64);
    }
}
