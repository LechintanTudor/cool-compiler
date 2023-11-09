mod ty_config;
mod ty_def;
mod ty_factory;
mod ty_kind;

pub use self::ty_config::*;
pub use self::ty_def::*;
pub use self::ty_factory::*;
pub use self::ty_kind::*;

use crate::{ResolveContext, ResolveError, ResolveResult};
use cool_collections::{define_index_newtype, SmallVec};
use cool_derive::define_tys;
use cool_lexer::{sym, Symbol};
use std::fmt::Write;

define_index_newtype!(TyId);

impl TyId {
    #[inline]
    #[must_use]
    pub fn is_definable(&self) -> bool {
        ![tys::infer, tys::infer_number, tys::alias, tys::module].contains(self)
    }
}

define_tys! {
    // Undefined types
    infer,
    infer_number,

    alias,
    module,

    // Defined types
    unit,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
}

impl ResolveContext<'_> {
    pub(crate) fn init_builtins(&mut self) {
        // Undefined
        debug_assert_eq!(self.tys.insert(InferTy::Any.into()), tys::infer);
        debug_assert_eq!(self.tys.insert(InferTy::Number.into()), tys::infer_number);

        debug_assert_eq!(self.tys.insert(ItemTy::Alias.into()), tys::alias);
        debug_assert_eq!(self.tys.insert(ItemTy::Module.into()), tys::module);

        // Unit
        let unit_ty_id = self.tys.insert(TyKind::Unit);
        debug_assert_eq!(unit_ty_id, tys::unit);

        let unit_def = self.define_ty(unit_ty_id);
        debug_assert!(unit_def.is_ok());

        // Signed integers
        self.add_primitive_ty(sym::i8, tys::i8, IntTy::I8);
        self.add_primitive_ty(sym::i16, tys::i16, IntTy::I16);
        self.add_primitive_ty(sym::i32, tys::i32, IntTy::I32);
        self.add_primitive_ty(sym::i64, tys::i64, IntTy::I64);
        self.add_primitive_ty(sym::i128, tys::i128, IntTy::I128);
        self.add_primitive_ty(sym::isize, tys::isize, IntTy::Isize);

        // Unsigned integers
        self.add_primitive_ty(sym::u8, tys::u8, IntTy::U8);
        self.add_primitive_ty(sym::u16, tys::u16, IntTy::U16);
        self.add_primitive_ty(sym::u32, tys::u32, IntTy::U32);
        self.add_primitive_ty(sym::u64, tys::u64, IntTy::U64);
        self.add_primitive_ty(sym::u128, tys::u128, IntTy::U128);
        self.add_primitive_ty(sym::usize, tys::usize, IntTy::Usize);

        // Floats
        self.add_primitive_ty(sym::f32, tys::f32, FloatTy::F32);
        self.add_primitive_ty(sym::f64, tys::f64, FloatTy::F64);
    }

    fn add_primitive_ty<K>(&mut self, symbol: Symbol, ty_id: TyId, kind: K)
    where
        K: Into<TyKind>,
    {
        let item_id = self.paths.insert_slice(&[symbol]);
        let actual_ty_id = self.tys.insert(kind.into());

        debug_assert_eq!(actual_ty_id, ty_id);
        self.items.insert(item_id, actual_ty_id.into());

        let def = self.define_ty(ty_id);
        debug_assert!(def.is_ok());
    }

    pub fn define_ty(&mut self, ty_id: TyId) -> ResolveResult<&TyDef> {
        if self.ty_defs.contains_key(&ty_id) {
            return Ok(&self.ty_defs[&ty_id]);
        }

        let def = match self.tys[ty_id] {
            TyKind::Unit => TyDef::basic(0),
            TyKind::Int(int_ty) => {
                let size = match int_ty {
                    IntTy::I8 | IntTy::U8 => 1,
                    IntTy::I16 | IntTy::U16 => 2,
                    IntTy::I32 | IntTy::U32 => 4,
                    IntTy::I64 | IntTy::U64 => 8,
                    IntTy::I128 | IntTy::U128 => 16,
                    IntTy::Isize | IntTy::Usize => self.ty_config.ptr_size,
                };

                TyDef::basic(size)
            }
            TyKind::Float(float_ty) => {
                let size = match float_ty {
                    FloatTy::F32 => 4,
                    FloatTy::F64 => 8,
                };

                TyDef::basic(size)
            }
            TyKind::Ptr(_) | TyKind::ManyPtr(_) | TyKind::Fn(_) => {
                TyDef::basic(self.ty_config.ptr_size)
            }
            TyKind::Array(array_ty) => {
                let elem_def = self.define_ty(array_ty.elem_ty)?;

                TyDef {
                    align: elem_def.align,
                    size: elem_def.size * array_ty.len,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Slice(slice_ty) => {
                let fields = [
                    (
                        sym::ptr,
                        self.add_many_ptr_ty(slice_ty.elem_ty, slice_ty.is_mutable),
                    ),
                    (sym::len, tys::usize),
                ];

                return self.define_aggregate_ty(ty_id, &fields);
            }
            TyKind::Tuple(ref tuple_ty) => {
                let mut buffer = String::new();

                let fields = tuple_ty
                    .elem_tys
                    .iter()
                    .enumerate()
                    .map(|(index, ty_id)| {
                        write!(&mut buffer, "{index}").unwrap();
                        let field_symbol = Symbol::insert(&buffer);
                        buffer.clear();

                        (field_symbol, *ty_id)
                    })
                    .collect::<SmallVec<_, 8>>();

                return self.define_aggregate_ty(ty_id, &fields);
            }
            _ => return Err(ResolveError::TyIsIncomplete { ty_id }),
        };

        self.ty_defs.insert(ty_id, def);
        Ok(&self.ty_defs[&ty_id])
    }

    #[inline]
    pub fn iter_undefined_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.tys
            .iter_indexes()
            .filter(|ty_id| ty_id.is_definable() && !self.ty_defs.contains_key(ty_id))
    }

    #[inline]
    #[must_use]
    pub fn get_ty_def(&self, ty_id: TyId) -> Option<&TyDef> {
        self.ty_defs.get(&ty_id)
    }
}
