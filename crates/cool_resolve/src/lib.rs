mod error;
mod item;
mod ty;
mod value;

pub use self::error::*;
pub use self::item::*;
pub use self::ty::*;
pub use self::value::*;

use cool_collections::{Arena, VecMap};
use cool_lexer::{sym, Symbol};
use std::ops::Index;

#[derive(Debug)]
pub struct ResolveContext {
    ty_config: TyConfig,
    tys: Arena<TyId, TyKind>,
    ty_defs: VecMap<TyId, Option<TyDef>>,
    items: Arena<ItemId, (CrateId, CrateItemId)>,
    item_defs: VecMap<ItemId, Item>,
    crates: VecMap<CrateId, Crate>,
    modules: VecMap<ModuleId, Module>,
    bindings: VecMap<BindingId, Binding>,
}

impl ResolveContext {
    pub fn new(ty_config: TyConfig) -> Self {
        let mut context = Self {
            ty_config,
            tys: Arena::default(),
            ty_defs: VecMap::default(),
            items: Arena::default(),
            item_defs: VecMap::default(),
            crates: VecMap::default(),
            modules: VecMap::default(),
            bindings: VecMap::default(),
        };

        context.add_crate(Symbol::insert("@builtins"));

        context.add_nonitem_ty(tys::infer, InferTy::Any);
        context.add_nonitem_ty(tys::infer_number, InferTy::Number);
        context.add_nonitem_ty(tys::infer_int, InferTy::Int);
        context.add_nonitem_ty(tys::infer_int_or_bool, InferTy::IntOrBool);

        context.add_item_ty(sym::kw_alias, tys::alias, ItemTy::Alias);
        context.add_item_ty(sym::kw_module, tys::module, ItemTy::Module);

        context.add_nonitem_ty(tys::unit, TyKind::Unit);
        context.add_item_ty(sym::bool, tys::bool, TyKind::Bool);
        context.add_item_ty(sym::char, tys::char, TyKind::Char);

        context.add_item_ty(sym::i8, tys::i8, IntTy::I8);
        context.add_item_ty(sym::i16, tys::i16, IntTy::I16);
        context.add_item_ty(sym::i32, tys::i32, IntTy::I32);
        context.add_item_ty(sym::i64, tys::i64, IntTy::I64);
        context.add_item_ty(sym::i128, tys::i128, IntTy::I128);
        context.add_item_ty(sym::isize, tys::isize, IntTy::Isize);

        context.add_item_ty(sym::u8, tys::u8, IntTy::U8);
        context.add_item_ty(sym::u16, tys::u16, IntTy::U16);
        context.add_item_ty(sym::u32, tys::u32, IntTy::U32);
        context.add_item_ty(sym::u64, tys::u64, IntTy::U64);
        context.add_item_ty(sym::u128, tys::u128, IntTy::U128);
        context.add_item_ty(sym::usize, tys::usize, IntTy::Usize);

        context.add_item_ty(sym::f32, tys::f32, FloatTy::F32);
        context.add_item_ty(sym::f64, tys::f64, FloatTy::F64);

        context
    }

    fn add_nonitem_ty<K>(&mut self, expected_ty_id: TyId, ty_kind: K)
    where
        K: Into<TyKind>,
    {
        let ty_id = self.add_ty(ty_kind.into());
        debug_assert_eq!(ty_id, expected_ty_id);

        if ty_id.is_definable() {
            let ty_def = self.define_ty(ty_id);
            debug_assert!(ty_def.is_some());
        }
    }

    fn add_item_ty<K>(&mut self, symbol: Symbol, expected_ty_id: TyId, ty_kind: K)
    where
        K: Into<TyKind>,
    {
        let ty_id = self.add_ty(ty_kind.into());
        debug_assert_eq!(ty_id, expected_ty_id);

        self.add_item(ModuleId::BUILTINS, true, symbol, |_| ty_id)
            .unwrap();

        if ty_id.is_definable() {
            let ty_def = self.define_ty(ty_id);
            debug_assert!(ty_def.is_some());
        }
    }
}

impl Index<ItemId> for ResolveContext {
    type Output = Item;

    #[inline]
    #[must_use]
    fn index(&self, item_id: ItemId) -> &Self::Output {
        &self.item_defs[item_id]
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
