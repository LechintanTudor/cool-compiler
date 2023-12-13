mod error;
mod item;
mod ty;

pub use self::error::*;
pub use self::item::*;
pub use self::ty::*;

use cool_collections::{Arena, VecMap};
use cool_lexer::{sym, Symbol};
use std::ops::Index;

#[derive(Debug)]
pub struct ResolveContext {
    crates: VecMap<CrateId, Crate>,
    modules: VecMap<ModuleId, Module>,
    ty_config: TyConfig,
    tys: Arena<TyId, TyKind>,
}

impl ResolveContext {
    pub fn new(ty_config: TyConfig) -> Self {
        let mut context = Self {
            crates: VecMap::default(),
            modules: VecMap::default(),
            ty_config,
            tys: Arena::default(),
        };

        context.add_crate(Symbol::insert("@builtins"));

        context.add_nonitem_ty(tys::infer, InferTy::Any);
        context.add_nonitem_ty(tys::infer_number, InferTy::Number);
        context.add_nonitem_ty(tys::infer_int, InferTy::Int);
        context.add_nonitem_ty(tys::infer_int_or_bool, InferTy::IntOrBool);

        context.add_builtin_ty(sym::kw_alias, tys::alias, ItemTy::Alias);
        context.add_builtin_ty(sym::kw_module, tys::module, ItemTy::Module);

        context.add_nonitem_ty(tys::unit, TyKind::Unit);
        context.add_builtin_ty(sym::bool, tys::bool, TyKind::Bool);
        context.add_builtin_ty(sym::char, tys::char, TyKind::Char);

        context.add_builtin_ty(sym::i8, tys::i8, IntTy::I8);
        context.add_builtin_ty(sym::i16, tys::i16, IntTy::I16);
        context.add_builtin_ty(sym::i32, tys::i32, IntTy::I32);
        context.add_builtin_ty(sym::i64, tys::i64, IntTy::I64);
        context.add_builtin_ty(sym::i128, tys::i128, IntTy::I128);
        context.add_builtin_ty(sym::isize, tys::isize, IntTy::Isize);

        context.add_builtin_ty(sym::u8, tys::u8, IntTy::U8);
        context.add_builtin_ty(sym::u16, tys::u16, IntTy::U16);
        context.add_builtin_ty(sym::u32, tys::u32, IntTy::U32);
        context.add_builtin_ty(sym::u64, tys::u64, IntTy::U64);
        context.add_builtin_ty(sym::u128, tys::u128, IntTy::U128);
        context.add_builtin_ty(sym::usize, tys::usize, IntTy::Usize);

        context.add_builtin_ty(sym::f32, tys::f32, FloatTy::F32);
        context.add_builtin_ty(sym::f64, tys::f64, FloatTy::F64);

        context
    }

    fn add_nonitem_ty<K>(&mut self, expected_ty_id: TyId, infer_ty: K)
    where
        K: Into<TyKind>,
    {
        let ty_id = self.tys.insert(infer_ty.into());
        debug_assert_eq!(ty_id, expected_ty_id);
    }

    fn add_builtin_ty<K>(&mut self, symbol: Symbol, expected_ty_id: TyId, ty_kind: K)
    where
        K: Into<TyKind>,
    {
        let ty_id = self.tys.insert(ty_kind.into());
        debug_assert_eq!(ty_id, expected_ty_id);

        let module_id = self.crates[CrateId::BUILTINS].module_id;
        self.add_item(module_id, true, symbol, ty_id.into())
            .unwrap();
    }
}

impl Index<CrateId> for ResolveContext {
    type Output = Crate;

    #[inline]
    #[must_use]
    fn index(&self, crate_id: CrateId) -> &Self::Output {
        &self.crates[crate_id]
    }
}

impl Index<ModuleId> for ResolveContext {
    type Output = Module;

    #[inline]
    #[must_use]
    fn index(&self, module_id: ModuleId) -> &Self::Output {
        &self.modules[module_id]
    }
}

impl Index<TyId> for ResolveContext {
    type Output = TyKind;

    #[inline]
    #[must_use]
    fn index(&self, ty_id: TyId) -> &Self::Output {
        &self.tys[ty_id]
    }
}
