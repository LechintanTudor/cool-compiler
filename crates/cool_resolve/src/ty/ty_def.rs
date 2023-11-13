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
    pub const fn basic(size: u64) -> Self {
        Self {
            size,
            align: size,
            kind: TyDefKind::Basic,
        }
    }

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
    pub is_nullable_ptr: bool,
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
    pub fn define_aggregate_ty(
        &mut self,
        ty_id: TyId,
        fields: &[(Symbol, TyId)],
    ) -> ResolveResult<&TyDef> {
        let mut fields = fields
            .iter()
            .enumerate()
            .map(|(index, (symbol, ty_id))| {
                self.define_ty(*ty_id)
                    .cloned()
                    .map(|ty_def| (index, *symbol, *ty_id, ty_def))
            })
            .collect::<Result<SmallVec<_, 8>, _>>()?;

        fields.sort_by_key(|(_, _, _, ty_def)| Reverse((ty_def.align, ty_def.size)));

        let mut offset = 0;
        let mut align = 1;

        let mut fields = fields
            .iter()
            .map(|(index, symbol, ty_id, ty_def)| {
                let field = Field {
                    offset,
                    symbol: *symbol,
                    ty_id: *ty_id,
                };

                offset += ty_def.size + compute_padding_for_align(offset, ty_def.align);
                align = align.max(ty_def.align);
                (index, field)
            })
            .collect::<SmallVec<_, 8>>();

        fields.sort_by_key(|(index, _)| *index);

        let fields = fields
            .into_iter()
            .map(|(_, field)| field)
            .collect::<Vec<_>>();

        self.ty_defs.insert(
            ty_id,
            TyDef {
                size: offset + compute_padding_for_align(offset, align),
                align,
                kind: AggregateTyDef { fields }.into(),
            },
        );

        Ok(&self.ty_defs[&ty_id])
    }

    pub fn define_variant_ty(
        &mut self,
        ty_id: TyId,
        variant_tys: &[TyId],
    ) -> ResolveResult<&TyDef> {
        let variant_ty_defs = variant_tys
            .iter()
            .map(|ty_id| {
                self.define_ty(*ty_id)
                    .cloned()
                    .map(|ty_def| (*ty_id, ty_def))
            })
            .collect::<Result<SmallVec<_, 8>, _>>()?;

        assert!(variant_ty_defs.len() <= 255);

        if variant_tys.len() == 2 {
            if let Some(ty_def) =
                self.compute_nullable_ptr_ty(&variant_ty_defs[0], &variant_ty_defs[1])
            {
                drop(variant_ty_defs);
                self.ty_defs.insert(ty_id, ty_def);
                return Ok(&self.ty_defs[&ty_id]);
            }
        }

        let offset = 1 + variant_ty_defs
            .iter()
            .map(|(_, ty_def)| ty_def.size)
            .max()
            .unwrap_or(0);

        let align = variant_ty_defs
            .iter()
            .map(|(_, ty_def)| ty_def.align)
            .max()
            .unwrap_or(1);

        drop(variant_ty_defs);

        self.ty_defs.insert(
            ty_id,
            TyDef {
                size: offset + compute_padding_for_align(offset, align),
                align,
                kind: VariantTyDef {
                    variant_tys: variant_tys.iter().cloned().collect(),
                    is_nullable_ptr: false,
                }
                .into(),
            },
        );

        Ok(&self.ty_defs[&ty_id])
    }

    fn compute_nullable_ptr_ty(
        &self,
        (ty_id_1, ty_def_1): &(TyId, TyDef),
        (ty_id_2, ty_def_2): &(TyId, TyDef),
    ) -> Option<TyDef> {
        let (ptr_ty, zero_sized_ty, ty_def) =
            if self.tys[*ty_id_1].is_ptr_or_slice() && ty_def_2.is_zero_sized() {
                (*ty_id_1, *ty_id_2, ty_def_1)
            } else if self.tys[*ty_id_2].is_ptr_or_slice() && ty_def_1.is_zero_sized() {
                (*ty_id_2, *ty_id_1, ty_def_2)
            } else {
                return None;
            };

        Some(TyDef {
            size: ty_def.size,
            align: ty_def.align,
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
