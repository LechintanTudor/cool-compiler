use crate::{AnyTy, Field, ItemId, ManyPtrTy, StructTy, TupleTy, TyContext, TyId, ValueTy};
use std::cmp::Reverse;

#[derive(Clone, Copy, Debug)]
pub struct PrimitiveTys {
    pub i8_align: u64,
    pub i16_align: u64,
    pub i32_align: u64,
    pub i64_align: u64,
    pub i128_align: u64,
    pub f32_align: u64,
    pub f64_align: u64,
    pub ptr_size: u64,
    pub ptr_align: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolveTy {
    pub size: u64,
    pub align: u64,
    pub ty: ValueTy,
}

impl ResolveTy {
    #[inline]
    pub fn is_zero_sized(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn is_comparable(&self) -> bool {
        matches!(
            self.ty,
            ValueTy::Int(_)
                | ValueTy::Float(_)
                | ValueTy::Bool
                | ValueTy::Char
                | ValueTy::Ptr(_)
                | ValueTy::ManyPtr(_),
        )
    }
}
impl TyContext {
    pub fn declare_struct(&mut self, item_id: ItemId) -> TyId {
        self.tys.get_or_insert(AnyTy::StructDecl(item_id))
    }

    #[must_use]
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

    pub fn value_ty_to_resolve_ty(&mut self, ty: ValueTy) -> Option<ResolveTy> {
        let ty: ResolveTy = match ty {
            ValueTy::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: ValueTy::Unit,
                }
            }
            ValueTy::Int(int_ty) => int_ty.to_resolve_ty(&self.primitives),
            ValueTy::Float(float_ty) => float_ty.to_resolve_ty(&self.primitives),
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
            ValueTy::Array(array_ty) => {
                let elem = self.get_resolve_ty(array_ty.elem)?;

                ResolveTy {
                    size: elem.size * array_ty.len,
                    align: elem.align,
                    ty: ValueTy::Array(array_ty),
                }
            }
            ValueTy::Ptr(_) | ValueTy::ManyPtr(_) | ValueTy::Fn(_) => {
                ResolveTy {
                    size: self.primitives.ptr_size,
                    align: self.primitives.ptr_align,
                    ty,
                }
            }
            ValueTy::Slice(slice_ptr_ty) => {
                self.tys
                    .get_or_insert(AnyTy::Value(ValueTy::ManyPtr(ManyPtrTy {
                        is_mutable: slice_ptr_ty.is_mutable,
                        pointee: slice_ptr_ty.elem,
                    })));

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
                let (size, align, fields) = self.resolve_size_align_fields(&tuple_ty.fields)?;

                ResolveTy {
                    size,
                    align,
                    ty: ValueTy::Tuple(TupleTy { fields }),
                }
            }
            ValueTy::Struct(struct_ty) => {
                let (size, align, fields) = self.resolve_size_align_fields(&struct_ty.fields)?;

                ResolveTy {
                    size,
                    align,
                    ty: ValueTy::Struct(StructTy {
                        item_id: struct_ty.item_id,
                        fields,
                    }),
                }
            }
        };

        Some(ty)
    }

    fn resolve_size_align_fields(&self, fields: &[Field]) -> Option<(u64, u64, Vec<Field>)> {
        let mut fields = fields
            .iter()
            .map_while(|field| self.get_resolve_ty(field.ty_id).map(|ty| (ty, field)))
            .collect::<Vec<_>>();

        fields.sort_by_key(|(ty, _)| Reverse(ty.align));

        let mut offset = 0;
        let mut align = 1;

        let fields = fields
            .iter()
            .map(|(ty, field)| {
                let field_offset = offset;

                offset += compute_padding_for_align(offset, ty.align) + ty.size;
                align = align.max(ty.align);

                Field {
                    offset: field_offset,
                    symbol: field.symbol,
                    ty_id: field.ty_id,
                }
            })
            .collect::<Vec<_>>();

        let size = offset + compute_padding_for_align(offset, align);
        Some((size, align, fields))
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
