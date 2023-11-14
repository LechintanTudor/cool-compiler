use crate::{ResolveContext, ResolveResult, TyId};
use cool_collections::SmallVec;
use cool_lexer::Symbol;
use derive_more::From;
use std::cmp::Reverse;

#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyDefKind,
}

impl TyDef {
    #[inline]
    #[must_use]
    pub const fn is_zero_sized(&self) -> bool {
        self.size == 0
    }
}

#[derive(Clone, From, Debug)]
pub enum TyDefKind {
    Basic,
    Aggregate(AggregateTyDef),
    Variant(VariantTyDef),
    NullablePtr(NullablePtrTyDef),
}

#[derive(Clone, Debug)]
pub struct AggregateTyDef {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct VariantTyDef {
    pub variant_tys: SmallVec<TyId, 4>,
}

#[derive(Clone, Debug)]
pub struct NullablePtrTyDef {
    pub ptr_ty: TyId,
    pub zero_sized_ty: TyId,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

impl ResolveContext<'_> {
    pub fn compute_aggregate_ty_def<F>(&mut self, fields: F) -> ResolveResult<TyDef>
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        let mut fields = fields
            .into_iter()
            .enumerate()
            .map(|(index, (symbol, ty_id))| {
                self.define_ty(ty_id)
                    .map(|ty_def| (index, symbol, ty_id, ty_def.size, ty_def.align))
            })
            .collect::<Result<SmallVec<_, 8>, _>>()?;

        fields.sort_by_key(|(_, _, _, size, align)| Reverse((*align, *size)));

        let mut offset = 0;
        let mut align = 1;

        let mut fields = fields
            .into_iter()
            .map(|(index, symbol, ty_id, field_size, field_align)| {
                let field = Field {
                    offset,
                    symbol,
                    ty_id,
                };

                offset += field_size + compute_padding_for_align(offset, field_align);
                align = align.max(field_align);

                (index, field)
            })
            .collect::<SmallVec<_, 8>>();

        fields.sort_by_key(|(index, _)| *index);

        let fields = fields
            .into_iter()
            .map(|(_, field)| field)
            .collect::<Vec<_>>();

        Ok(TyDef {
            size: offset + compute_padding_for_align(offset, align),
            align,
            kind: AggregateTyDef { fields }.into(),
        })
    }

    pub fn compute_variant_ty_def<V>(&mut self, variant_tys: V) -> ResolveResult<TyDef>
    where
        V: IntoIterator<Item = TyId>,
    {
        let variant_tys = variant_tys.into_iter().collect::<SmallVec<_, 4>>();

        let variant_ty_defs = variant_tys
            .iter()
            .map(|&ty_id| {
                self.define_ty(ty_id)
                    .map(|ty_def| (ty_id, ty_def.size, ty_def.align))
            })
            .collect::<Result<SmallVec<_, 8>, _>>()?;

        assert!(variant_ty_defs.len() <= 255);

        if variant_ty_defs.len() == 2 {
            let (ty_id_1, ty_size_1, ty_align_1) = variant_ty_defs[0];
            let (ty_id_2, ty_size_2, ty_align_2) = variant_ty_defs[1];

            if let Some(ty_def) = self.try_compute_nullable_ptr_ty_def(
                ty_id_1, ty_size_1, ty_align_1, ty_id_2, ty_size_2, ty_align_2,
            ) {
                return Ok(ty_def);
            }
        }

        let (offset, align) = variant_ty_defs
            .iter()
            .map(|(_, size, align)| (*size, *align))
            .fold((0, 1), |(s, a), (size, align)| (s.max(size), a.max(align)));

        Ok(TyDef {
            size: compute_padding_for_align(offset, align),
            align,
            kind: VariantTyDef { variant_tys }.into(),
        })
    }

    fn try_compute_nullable_ptr_ty_def(
        &self,
        ty_id_1: TyId,
        ty_size_1: u64,
        ty_align_1: u64,
        ty_id_2: TyId,
        ty_size_2: u64,
        ty_align_2: u64,
    ) -> Option<TyDef> {
        let (ptr_ty, zero_sized_ty, size, align) =
            if self.tys[ty_id_1].is_ptr_or_slice() && ty_size_2 == 0 {
                (ty_id_1, ty_id_2, ty_size_1, ty_align_1)
            } else if self.tys[ty_id_2].is_ptr_or_slice() && ty_size_2 == 0 {
                (ty_id_2, ty_id_1, ty_size_2, ty_align_2)
            } else {
                return None;
            };

        Some(TyDef {
            size,
            align,
            kind: NullablePtrTyDef {
                ptr_ty,
                zero_sized_ty,
            }
            .into(),
        })
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
