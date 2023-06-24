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
mod ty_def;
mod ty_id;
mod ty_shape;
mod value_ty;
mod variant_ty;

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
pub use self::ty_def::*;
pub use self::ty_id::*;
pub use self::ty_shape::*;
pub use self::value_ty::*;
pub use self::variant_ty::*;
use cool_arena::InternArena;
use cool_lexer::{sym, Symbol};
use rustc_hash::FxHashSet;
use smallvec::SmallVec;

pub(crate) type TyArena = InternArena<'static, ResolveTy>;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyData,
    tys: TyArena,
    undefined_tys: FxHashSet<TyId>,
    consts: TyConsts,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTyData) -> Self {
        let mut tys = InternArena::new_leak();
        let consts = TyConsts::blank(&mut tys);

        let mut tys = Self {
            primitives,
            tys,
            undefined_tys: Default::default(),
            consts,
        };
        tys.consts = TyConsts::new(&mut tys);
        tys
    }

    pub fn insert(&mut self, ty_shape: TyShape) -> TyId {
        let ty_def = match &ty_shape {
            TyShape::Value(value_ty) => {
                self.value_ty_to_ty_def(value_ty)
                    .unwrap_or_else(TyDef::deferred)
            }
            _ => TyDef::Undefined,
        };

        TyId::from(self.tys.insert(ResolveTy {
            shape: ty_shape,
            def: ty_def,
        }))
    }

    pub fn insert_value<T>(&mut self, value_ty: T) -> TyId
    where
        T: Into<ValueTy>,
    {
        let value_ty: ValueTy = value_ty.into();
        self.insert(value_ty.into())
    }

    pub fn value_ty_to_ty_def(&mut self, value_ty: &ValueTy) -> Option<TyDef> {
        let ty_def = match value_ty {
            ValueTy::Unit => TyDef::from(BasicTyDef { size: 0, align: 1 }),
            ValueTy::Bool => TyDef::from(BasicTyDef { size: 1, align: 1 }),
            ValueTy::Char => {
                TyDef::from(BasicTyDef {
                    size: 4,
                    align: self.primitives.i32_align,
                })
            }
            ValueTy::Int(int_ty) => int_ty.to_ty_def(&self.primitives),
            ValueTy::Float(float_ty) => float_ty.to_ty_def(&self.primitives),
            ValueTy::Tuple(tuple_ty) => {
                let fields = tuple_ty
                    .elems
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (Symbol::insert_u32(i as u32), *ty))
                    .collect::<SmallVec<[_; 2]>>();

                TyDef::aggregate(fields).ok()?
            }
            ValueTy::Struct(_) => TyDef::deferred(),
            ValueTy::Fn(fn_ty) => fn_ty.to_ty_def(&self.primitives),
            ValueTy::Ptr(ptr_ty) => ptr_ty.to_ty_def(&self.primitives),
            ValueTy::ManyPtr(many_ptr_ty) => many_ptr_ty.to_ty_def(&self.primitives),
            ValueTy::Slice(slice_ty) => {
                let fields = vec![
                    (
                        sym::PTR,
                        self.insert_value(ManyPtrTy {
                            pointee: slice_ty.elem,
                            is_mutable: slice_ty.is_mutable,
                        }),
                    ),
                    (sym::LEN, self.consts.usize),
                ];

                TyDef::aggregate(fields).ok()?
            }
            _ => todo!("{}", value_ty),
        };

        Some(ty_def)
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
        if found_ty_id.shape.is_diverge() {
            return Some(expected_ty_id);
        }

        let tys = &self.consts;

        let ty_id = if expected_ty_id == tys.infer {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if !found_ty_id.shape.is_infer() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_number {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id == tys.infer_float {
                tys.f32
            } else if found_ty_id.shape.is_number() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_int {
            if found_ty_id == tys.infer_int {
                tys.i32
            } else if found_ty_id.shape.is_int() {
                found_ty_id
            } else {
                return None;
            }
        } else if expected_ty_id == tys.infer_float {
            if found_ty_id == tys.infer_int {
                tys.f64
            } else if found_ty_id == tys.infer_float {
                tys.f64
            } else if found_ty_id.shape.is_float() {
                found_ty_id
            } else {
                return None;
            }
        } else {
            let can_resolve_directly = (found_ty_id == expected_ty_id)
                || (found_ty_id == tys.infer)
                || (found_ty_id == tys.infer_number && expected_ty_id.shape.is_number())
                || (found_ty_id == tys.infer_int && expected_ty_id.shape.is_number())
                || (found_ty_id == tys.infer_float && expected_ty_id.shape.is_float())
                || (found_ty_id == tys.infer_empty_array && expected_ty_id.shape.is_array());

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
        self.tys
            .iter()
            .filter(|ty| ty.shape.is_value())
            .map(TyId::from)
    }
}
