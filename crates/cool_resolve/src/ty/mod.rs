mod ty_config;
mod ty_def;
mod ty_error;
mod ty_kind;

pub use self::ty_config::*;
pub use self::ty_def::*;
pub use self::ty_error::*;
pub use self::ty_kind::*;

use crate::ResolveContext;
use cool_arena::define_arena_index;
use cool_derive::define_tys;
use cool_lexer::{sym, Symbol};

define_arena_index!(TyId);

define_tys! {
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
        // Unit
        let unit_ty_id = self.tys.insert(TyKind::Unit);
        debug_assert_eq!(unit_ty_id, tys::unit);

        let unit_def = self.define_ty(unit_ty_id);
        debug_assert!(unit_def.is_ok());

        // Signed integers
        self.insert_primitive_ty(sym::i8, tys::i8, IntTy::I8);
        self.insert_primitive_ty(sym::i16, tys::i16, IntTy::I16);
        self.insert_primitive_ty(sym::i32, tys::i32, IntTy::I32);
        self.insert_primitive_ty(sym::i64, tys::i64, IntTy::I64);
        self.insert_primitive_ty(sym::i128, tys::i128, IntTy::I128);
        self.insert_primitive_ty(sym::isize, tys::isize, IntTy::Isize);

        // Unsigned integers
        self.insert_primitive_ty(sym::u8, tys::u8, IntTy::U8);
        self.insert_primitive_ty(sym::u16, tys::u16, IntTy::U16);
        self.insert_primitive_ty(sym::u32, tys::u32, IntTy::U32);
        self.insert_primitive_ty(sym::u64, tys::u64, IntTy::U64);
        self.insert_primitive_ty(sym::u128, tys::u128, IntTy::U128);
        self.insert_primitive_ty(sym::usize, tys::usize, IntTy::Usize);

        // Floats
        self.insert_primitive_ty(sym::f32, tys::f32, FloatTy::F32);
        self.insert_primitive_ty(sym::f64, tys::f64, FloatTy::F64);
    }

    fn insert_primitive_ty<K>(&mut self, symbol: Symbol, ty_id: TyId, kind: K)
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

    fn define_ty(&mut self, ty_id: TyId) -> TyResult<&TyDef> {
        if self.ty_defs.contains_key(&ty_id) {
            return Ok(&self.ty_defs[&ty_id]);
        }

        let def = match &self.tys[ty_id] {
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
            _ => return Err(TyError::CannotBeDefined),
        };

        self.ty_defs.insert(ty_id, def);
        Ok(&self.ty_defs[&ty_id])
    }
}
