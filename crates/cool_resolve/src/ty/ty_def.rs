use crate::{ResolveContext, ResolveResult, TyId};
use cool_lexer::Symbol;
use smallvec::SmallVec;
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
}

#[derive(Clone, Debug)]
pub enum TyDefKind {
    Basic,
    Aggregate(Vec<Field>),
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
            .collect::<Result<SmallVec<[_; 8]>, _>>()?;

        fields.sort_by_key(|(_, _, _, ty_def)| Reverse(ty_def.align));

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
            .collect::<SmallVec<[_; 8]>>();

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
                kind: TyDefKind::Aggregate(fields),
            },
        );

        Ok(&self.ty_defs[&ty_id])
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
