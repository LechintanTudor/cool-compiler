mod resolve_ty;
mod ty_error;
mod ty_id;
mod ty_kind;

pub use self::resolve_ty::*;
pub use self::ty_error::*;
pub use self::ty_id::*;
pub use self::ty_kind::*;
use crate::{tys, ItemId};
use cool_arena::Arena;
use cool_collections::IdIndexedVec;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyProps,
    ty_kinds: Arena<TyId, TyKind>,
    resolve_tys: IdIndexedVec<TyId, ResolveTy>,
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
            ty_kinds: Default::default(),
            resolve_tys: Default::default(),
        }
    }

    pub fn get_or_insert(&mut self, ty_kind: TyKind) -> TyId {
        if let Some(ty_id) = self.ty_kinds.get_id(&ty_kind) {
            return ty_id;
        }

        let resolve_ty = self.resolve_ty_kind(ty_kind.clone());
        let ty_id = self.ty_kinds.get_or_insert(ty_kind);
        self.resolve_tys.push_checked(ty_id, resolve_ty);
        ty_id
    }

    pub fn insert_builtin(&mut self, ty_id: TyId, ty_kind: TyKind) {
        let resolve_ty = self.resolve_ty_kind(ty_kind.clone());
        self.ty_kinds.insert_checked(ty_id, ty_kind);
        self.resolve_tys.push_checked(ty_id, resolve_ty);
    }

    pub fn declare_struct(&mut self, item_id: ItemId) -> TyId {
        let ty_id = self.ty_kinds.get_or_insert(TyKind::StructDecl(item_id));
        self.resolve_tys
            .push_checked(ty_id, ResolveTy::struct_decl(item_id));
        ty_id
    }

    pub fn define_struct(&mut self, struct_ty: StructTy) {
        let ty_id = self
            .ty_kinds
            .get_id(&TyKind::StructDecl(struct_ty.item_id))
            .unwrap();

        let resolve_ty = self.resolve_ty_kind(TyKind::Struct(struct_ty));

        let ty = &mut self.resolve_tys[ty_id];
        debug_assert!(matches!(&ty.kind, TyKind::StructDecl(_)));
        *ty = resolve_ty;
    }

    #[inline]
    pub fn get_resolve_ty(&self, ty_id: TyId) -> &ResolveTy {
        &self.resolve_tys[ty_id]
    }

    #[inline]
    pub fn iter_ty_ids(&self) -> impl Iterator<Item = TyId> {
        self.ty_kinds.iter_ids()
    }

    fn resolve_ty_kind(&self, ty_kind: TyKind) -> ResolveTy {
        match ty_kind {
            TyKind::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    kind: TyKind::Unit,
                }
            }
            TyKind::Int(int_ty) => {
                let mk_resolve_ty = |size, align| {
                    ResolveTy {
                        size,
                        align,
                        kind: ty_kind,
                    }
                };

                match int_ty {
                    IntTy::I8 | IntTy::U8 => mk_resolve_ty(1, self.primitives.i8_align),
                    IntTy::I16 | IntTy::U16 => mk_resolve_ty(2, self.primitives.i16_align),
                    IntTy::I32 | IntTy::U32 => mk_resolve_ty(4, self.primitives.i32_align),
                    IntTy::I64 | IntTy::U64 => mk_resolve_ty(8, self.primitives.i64_align),
                    IntTy::I128 | IntTy::U128 => mk_resolve_ty(16, self.primitives.i128_align),
                    IntTy::Isize | IntTy::Usize => {
                        mk_resolve_ty(self.primitives.ptr_size, self.primitives.ptr_align)
                    }
                }
            }
            TyKind::Float(float_ty) => {
                let mk_resolve_ty = |size, align| {
                    ResolveTy {
                        size,
                        align,
                        kind: ty_kind,
                    }
                };

                match float_ty {
                    FloatTy::F32 => mk_resolve_ty(4, self.primitives.f32_align),
                    FloatTy::F64 => mk_resolve_ty(8, self.primitives.f64_align),
                }
            }
            TyKind::Bool => {
                ResolveTy {
                    size: 1,
                    align: self.primitives.i8_align,
                    kind: TyKind::Bool,
                }
            }
            TyKind::Char => {
                ResolveTy {
                    size: 4,
                    align: self.primitives.i32_align,
                    kind: TyKind::Char,
                }
            }
            TyKind::Pointer(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    kind: ty_kind,
                }
            }
            TyKind::Fn(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    kind: ty_kind,
                }
            }
            TyKind::Array(array_ty) => {
                let elem = &self.get_resolve_ty(array_ty.elem);

                ResolveTy {
                    size: elem.size * array_ty.len,
                    align: elem.align,
                    kind: ty_kind,
                }
            }
            TyKind::Struct(struct_ty) => {
                let mut offset = 0;
                let mut align = 0;

                for (_, field_ty_id) in struct_ty.fields.iter() {
                    let field_ty = self.get_resolve_ty(*field_ty_id);
                    offset += compute_padding_for_align(offset, field_ty.align) + field_ty.size;
                    align = align.max(field_ty.align);
                }

                let size = offset + compute_padding_for_align(offset, align);

                ResolveTy {
                    size,
                    align,
                    kind: TyKind::Struct(struct_ty),
                }
            }
            TyKind::Infer(inferred_ty) => ResolveTy::inferred(inferred_ty),
            TyKind::Module => ResolveTy::module(),
            ty_kind => todo!("todo type resolve: {ty_kind:?}"),
        }
    }

    pub fn resolve_direct_ty_id(&self, found_ty_id: TyId, expected_ty_id: TyId) -> Option<TyId> {
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
            _ => {
                let can_resolve_directly = (found_ty_id == expected_ty_id)
                    || (found_ty_id == tys::INFER)
                    || (found_ty_id == tys::INFER_NUMBER && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_INT && expected_ty_id.is_number())
                    || (found_ty_id == tys::INFER_FLOAT && expected_ty_id.is_float());

                if can_resolve_directly {
                    return Some(expected_ty_id);
                }

                match self.get_resolve_ty(expected_ty_id).kind {
                    TyKind::Array(_) => {
                        if found_ty_id == tys::INFER_EMPTY_ARRAY {
                            Some(expected_ty_id)
                        } else {
                            None
                        }
                    }
                    TyKind::Pointer(pointer_ty) => {
                        let found_pointer_ty =
                            self.get_resolve_ty(found_ty_id).kind.as_pointer_ty()?;

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
