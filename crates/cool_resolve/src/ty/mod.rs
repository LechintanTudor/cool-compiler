mod consts;
mod def;
mod error;
mod shape;
mod ty_id;

pub use self::consts::*;
pub use self::def::*;
pub use self::error::*;
pub use self::shape::*;
pub use self::ty_id::*;
use cool_arena::InternArena;
use cool_lexer::Symbol;
use rustc_hash::FxHashMap;

pub(crate) type TyShapes = InternArena<'static, TyShape>;
pub(crate) type TyDefs = FxHashMap<TyId, TyDef>;

#[derive(Debug)]
pub struct TyContext {
    shapes: TyShapes,
    defs: TyDefs,
    primitives: PrimitiveTyData,
    consts: TyConsts,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTyData) -> Self {
        let mut shapes = TyShapes::new_leak();
        let mut defs = TyDefs::default();
        let consts = TyConsts::new(&mut shapes, &mut defs, &primitives);

        Self {
            shapes,
            defs,
            primitives,
            consts,
        }
    }

    pub fn insert(&mut self, ty_shape: TyShape) -> TyId {
        let ty_id = TyId::from(self.shapes.insert(ty_shape));
        let _ = self.define(ty_id);
        ty_id
    }

    pub fn insert_value<T>(&mut self, value_ty: T) -> TyId
    where
        T: Into<ValueTy>,
    {
        self.insert(TyShape::from(value_ty.into()))
    }

    pub fn define(&mut self, ty_id: TyId) -> TyResult<&TyDef> {
        if let Some(def) = self.defs.get(&ty_id) {
            return Ok(def);
        }

        let TyShape::Value(value_ty) = &*ty_id else {
            return Err(TyError {
                ty_id,
                kind: TyErrorKind::CannotBeDefined,
            });
        };

        let def = match value_ty {
            ValueTy::Unit => TyDef::for_unit(),
            ValueTy::Bool => TyDef::for_bool(&self.primitives),
            ValueTy::Char => TyDef::for_char(&self.primitives),
            ValueTy::Int(int_ty) => TyDef::for_int(*int_ty, &self.primitives),
            ValueTy::Float(float_ty) => TyDef::for_float(*float_ty, &self.primitives),
            ValueTy::Fn(_) | ValueTy::Ptr(_) | ValueTy::ManyPtr(_) => {
                TyDef::for_ptr(&self.primitives)
            }
            ValueTy::Array(array_ty) => {
                let elem_def = self.define(array_ty.elem)?;

                TyDef {
                    size: elem_def.size * array_ty.len,
                    align: elem_def.align,
                    kind: TyKind::Basic,
                }
            }
            _ => todo!(),
        };

        Ok(self.defs.entry(ty_id).or_insert(def))
    }

    pub fn define_struct<F>(&mut self, struct_ty_id: TyId, fields: F) -> TyResult
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        todo!()
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
        self.shapes
            .iter()
            .filter(|ty| ty.is_value())
            .map(TyId::from)
    }
}
