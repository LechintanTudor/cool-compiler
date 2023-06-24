mod any_ty;
mod array_ty;
mod consts;
mod enum_ty;
mod error;
mod field;
mod fn_ty;
mod infer_ty;
mod item_ty;
mod primitive_ty;
mod ptr_ty;
mod resolve_ty;
mod struct_ty;
mod tuple_ty;
mod ty_id;
mod value_ty;
mod variant_ty;

pub use self::any_ty::*;
pub use self::array_ty::*;
pub use self::consts::*;
pub use self::enum_ty::*;
pub use self::error::*;
pub use self::field::*;
pub use self::fn_ty::*;
pub use self::infer_ty::*;
pub use self::item_ty::*;
pub use self::primitive_ty::*;
pub use self::ptr_ty::*;
pub use self::resolve_ty::*;
pub use self::struct_ty::*;
pub use self::tuple_ty::*;
pub use self::ty_id::*;
pub use self::value_ty::*;
pub use self::variant_ty::*;
use crate::ItemId;
use cool_arena::Arena;
use cool_collections::id_newtype;

id_newtype!(InternalTyId);

pub(crate) type TyArena = Arena<'static, InternalTyId, ResolveTy>;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyData,
    tys: TyArena,
    consts: TyConsts,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTyData) -> Self {
        let mut tys = TyArena::new_leak();
        let consts = TyConsts::new(&mut tys, &primitives);

        Self {
            primitives,
            tys,
            consts,
        }
    }

    pub fn declare_struct(&mut self, item_id: ItemId) -> TyId {
        self.get_or_insert(AnyTy::from(ValueTy::from(StructTy {
            item_id,
            def: Default::default(),
        })))
    }

    pub fn get_or_insert(&mut self, ty: AnyTy) -> TyId {
        let ty = ty.to_resolve_ty(&self.primitives);
        let internal_ty_id = self.tys.insert(ty);
        TyId::new(self.tys.get(internal_ty_id).unwrap())
    }

    pub fn get_or_insert_value<T>(&mut self, ty: T) -> TyId
    where
        T: Into<ValueTy>,
    {
        let ty: ValueTy = ty.into();
        self.get_or_insert(ty.into())
    }

    pub fn resolve_direct_ty_id(
        &self,
        found_ty_id: TyId,
        expected_ty_id: TyId,
    ) -> Result<TyId, TyMismatch> {
        self.resolve_direct_ty_id_inner(found_ty_id, expected_ty_id)
            .ok_or(TyMismatch {
                found_ty_id,
                expected_ty_id,
            })
    }

    #[allow(clippy::if_same_then_else)]
    fn resolve_direct_ty_id_inner(&self, found_ty_id: TyId, expected_ty_id: TyId) -> Option<TyId> {
        if found_ty_id.is_diverge() {
            return Some(expected_ty_id);
        }

        let tys = &self.consts;

        let ty_id = if expected_ty_id == tys.infer {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if !found_ty_id.is_infer() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_number {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f32
            } else if found_ty_id.is_number() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_int {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id.is_int() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_float {
            if found_ty_id == tys.infer_int {
                tys.f64
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if found_ty_id.is_float() {
                found_ty_id
            } else {
                return None;
            }
        } else {
            let can_resolve_directly = (found_ty_id == expected_ty_id)
                || (found_ty_id == tys.infer)
                || (found_ty_id == tys.infer_number && expected_ty_id.is_number())
                || (found_ty_id == tys.infer_int && expected_ty_id.is_number())
                || (found_ty_id == tys.infer_float && expected_ty_id.is_float())
                || (found_ty_id == tys.infer_empty_array && expected_ty_id.is_array());

            if !can_resolve_directly {
                return None;
            }

            expected_ty_id
        };

        Some(ty_id)
    }

    #[inline]
    pub fn consts(&self) -> &TyConsts {
        &self.consts
    }

    #[inline]
    pub fn iter_value_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.tys.iter().filter(|ty| ty.ty.is_value()).map(TyId::new)
    }
}
