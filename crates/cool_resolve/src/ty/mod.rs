mod any_ty;
mod resolve_ty;
mod ty_error;
mod ty_id;
mod value_ty;

pub use self::any_ty::*;
pub use self::resolve_ty::*;
pub use self::ty_error::*;
pub use self::ty_id::*;
pub use self::value_ty::*;
use crate::{tys, ItemId};
use cool_arena::Arena;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyProps,
    tys: Arena<'static, TyId, AnyTy>,
    resolve_tys: FxHashMap<TyId, ResolveTy>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssignKind {
    Direct,
    MutPtrToPtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AssignTy {
    pub ty_id: TyId,
    pub kind: AssignKind,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTyProps) -> Self {
        Self {
            primitives,
            tys: Arena::new_leak(),
            resolve_tys: Default::default(),
        }
    }

    pub(crate) fn insert_builtin(&mut self, ty_id: TyId, ty: AnyTy) {
        self.tys.insert_checked(ty_id, ty.clone());

        if let AnyTy::Value(value) = ty {
            let resolve_ty = self.value_ty_to_resolve_ty(value).unwrap();
            self.resolve_tys.insert(ty_id, resolve_ty);
        }
    }

    pub fn get_or_insert(&mut self, ty: AnyTy) -> TyId {
        match ty {
            AnyTy::Value(value_ty) => {
                let ty_id = self.tys.get_or_insert(value_ty.clone().into());
                let resolve_ty = self.value_ty_to_resolve_ty(value_ty).unwrap();
                self.resolve_tys.insert(ty_id, resolve_ty);
                ty_id
            }
            _ => self.tys.get_or_insert(ty),
        }
    }

    pub fn declare_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.get_or_insert(AnyTy::StructDecl(item_id))
    }

    pub fn define_struct(&mut self, struct_ty: StructTy) -> bool {
        let ty_id = self
            .tys
            .get_id(&AnyTy::StructDecl(struct_ty.item_id))
            .unwrap();

        match self.value_ty_to_resolve_ty(ValueTy::Struct(struct_ty)) {
            Some(resolve_ty) => {
                self.resolve_tys.insert(ty_id, resolve_ty);
                true
            }
            None => false,
        }
    }

    fn value_ty_to_resolve_ty(&mut self, ty: ValueTy) -> Option<ResolveTy> {
        let ty: ResolveTy = match ty {
            ValueTy::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: ValueTy::Unit,
                }
            }
            ValueTy::Int(int_ty) => {
                let mk_int = |size, align| ResolveTy { size, align, ty };

                match int_ty {
                    IntTy::I8 | IntTy::U8 => mk_int(1, self.primitives.i8_align),
                    IntTy::I16 | IntTy::U16 => mk_int(2, self.primitives.i16_align),
                    IntTy::I32 | IntTy::U32 => mk_int(4, self.primitives.i32_align),
                    IntTy::I64 | IntTy::U64 => mk_int(8, self.primitives.i64_align),
                    IntTy::I128 | IntTy::U128 => mk_int(16, self.primitives.i128_align),
                    IntTy::Isize | IntTy::Usize => {
                        mk_int(self.primitives.ptr_size, self.primitives.ptr_align)
                    }
                }
            }
            ValueTy::Float(float_ty) => {
                let mk_float = |size, align| ResolveTy { size, align, ty };

                match float_ty {
                    FloatTy::F32 => mk_float(4, self.primitives.f32_align),
                    FloatTy::F64 => mk_float(8, self.primitives.f64_align),
                }
            }
            ValueTy::Bool => {
                ResolveTy {
                    size: 1,
                    align: 1,
                    ty: ValueTy::Bool,
                }
            }
            ValueTy::Char => {
                ResolveTy {
                    size: 4,
                    align: 4,
                    ty: ValueTy::Char,
                }
            }
            ValueTy::Fn(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    ty,
                }
            }
            ValueTy::Array(array_ty) => {
                let elem = self.get_resolve_ty(array_ty.elem)?;

                ResolveTy {
                    size: elem.size * array_ty.len,
                    align: elem.align,
                    ty: ValueTy::Array(array_ty),
                }
            }
            ValueTy::Ptr(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    ty,
                }
            }
            ValueTy::ManyPtr(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    ty,
                }
            }
            ValueTy::Slice(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size * 2,
                    align: self.primitives.ptr_align,
                    ty,
                }
            }
            ValueTy::Range => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty,
                }
            }
            ValueTy::Tuple(tuple_ty) => {
                let mut offset = 0;
                let mut align = 1;

                for elem_ty_id in tuple_ty.elems.iter() {
                    let elem_ty = self.get_resolve_ty(*elem_ty_id)?;
                    offset += compute_padding_for_align(offset, elem_ty.align) + elem_ty.size;
                    align = align.max(elem_ty.align);
                }

                let size = offset + compute_padding_for_align(offset, align);

                ResolveTy {
                    size,
                    align,
                    ty: ValueTy::Tuple(tuple_ty),
                }
            }
            ValueTy::Struct(struct_ty) => {
                let mut offset = 0;
                let mut align = 1;

                for (_, field_ty_id) in struct_ty.fields.iter() {
                    let field_ty = self.get_resolve_ty(*field_ty_id)?;
                    offset += compute_padding_for_align(offset, field_ty.align) + field_ty.size;
                    align = align.max(field_ty.align);
                }

                let size = offset + compute_padding_for_align(offset, align);

                ResolveTy {
                    size,
                    align,
                    ty: ValueTy::Struct(struct_ty),
                }
            }
        };

        Some(ty)
    }

    #[inline]
    pub fn get_resolve_ty(&self, ty_id: TyId) -> Option<&ResolveTy> {
        self.resolve_tys.get(&ty_id)
    }

    #[inline]
    pub fn iter_resolve_ty_ids(&self) -> impl Iterator<Item = TyId> + '_ {
        self.resolve_tys.keys().copied()
    }

    pub fn resolve_direct_ty_id(&self, found_ty_id: TyId, expected_ty_id: TyId) -> Option<TyId> {
        if found_ty_id.is_divergent() {
            return Some(expected_ty_id);
        }

        match expected_ty_id {
            tys::INFER => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    tys::INFER_FLOAT => tys::F64,
                    _ if !found_ty_id.is_inferred() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_NUMBER => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    tys::INFER_FLOAT => tys::F64,
                    _ if found_ty_id.is_number() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_INT => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::I32,
                    _ if found_ty_id.is_int() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_FLOAT => {
                let ty_id = match found_ty_id {
                    tys::INFER_INT => tys::F64,
                    tys::INFER_FLOAT => tys::F64,
                    _ if found_ty_id.is_float() => found_ty_id,
                    _ => return None,
                };

                Some(ty_id)
            }
            tys::INFER_SUBSCRIPT => {
                let ty_id = match found_ty_id {
                    tys::USIZE => tys::USIZE,
                    tys::RANGE_FULL => tys::RANGE_FULL,
                    tys::INFER_INT => tys::USIZE,
                    _ => return None,
                };

                Some(ty_id)
            }
            _ => {
                let can_resolve_directly = (found_ty_id == expected_ty_id)
                    || (found_ty_id == tys::INFER)
                    || (found_ty_id == tys::INFER_NUMBER && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_INT && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_FLOAT && expected_ty_id.is_float());

                if can_resolve_directly {
                    return Some(expected_ty_id);
                }

                match &self.get_resolve_ty(expected_ty_id)?.ty {
                    ValueTy::Array(_) => {
                        if found_ty_id == tys::INFER_EMPTY_ARRAY {
                            Some(expected_ty_id)
                        } else {
                            None
                        }
                    }
                    ValueTy::Ptr(pointer_ty) => {
                        let found_pointer_ty = self.get_resolve_ty(found_ty_id)?.ty.as_ptr()?;

                        let can_resolve = found_pointer_ty.pointee == pointer_ty.pointee
                            && !pointer_ty.is_mutable;

                        can_resolve.then_some(expected_ty_id)
                    }
                    _ => None,
                }
            }
        }
    }
}

fn compute_padding_for_align(offset: u64, align: u64) -> u64 {
    let misalign = offset % align;

    if misalign > 0 {
        align - misalign
    } else {
        0
    }
}
