mod resolve_ty;
mod ty_kind;

pub use self::resolve_ty::*;
pub use self::ty_kind::*;
use crate::{ItemId, TyId};
use cool_arena::Arena;
use cool_collections::IdIndexedVec;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyProps,
    ty_kinds: Arena<TyId, TyKind>,
    resolve_tys: IdIndexedVec<TyId, ResolveTy>,
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
            TyKind::Inferred(inferred_ty) => ResolveTy::inferred(inferred_ty),
            TyKind::Module => ResolveTy::module(),
            ty_kind => todo!("todo type resolve: {ty_kind:?}"),
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
