use crate::{TyContext, TyDef, TyError, TyErrorKind, TyId, TyKind, TyResult};
use cool_lexer::Symbol;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

#[derive(Clone, Debug)]
pub struct AggregateTy {
    fields: Arc<[Field]>,
}

impl AggregateTy {
    pub fn get_field(&self, symbol: Symbol) -> Option<&Field> {
        self.fields.iter().find(|field| field.symbol == symbol)
    }

    #[inline]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    #[inline]
    pub fn fields_arc(&self) -> &Arc<[Field]> {
        &self.fields
    }
}

impl TyContext {
    pub(crate) fn mk_aggregate_ty_def<F>(&mut self, ty_id: TyId, field_iter: F) -> TyResult<TyDef>
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        let mut fields = Vec::<(Symbol, TyId, &TyDef)>::new();
        let mut used_fields = FxHashSet::<Symbol>::default();

        for (field_symbol, field_ty_id) in field_iter {
            if !used_fields.insert(field_symbol) {
                return Err(TyError {
                    ty_id,
                    kind: TyErrorKind::StructHasDuplicatedField {
                        field: field_symbol,
                    },
                });
            }

            let Some(field_def) = self.get_def(field_ty_id) else {
                return Err(TyError {
                    ty_id,
                    kind: TyErrorKind::CannotBeDefined,
                });
            };

            fields.push((field_symbol, field_ty_id, field_def));
        }

        fields.sort_by_key(|(_, _, field_def)| Reverse(field_def.align));

        let mut offset = 0;
        let mut align = 1;

        let fields = fields
            .iter()
            .map(|(field_symbol, field_ty_id, field_def)| {
                let field = Field {
                    offset,
                    symbol: *field_symbol,
                    ty_id: *field_ty_id,
                };

                offset += compute_padding_for_align(offset, field_def.align);
                align = align.max(field_def.align);
                field
            })
            .collect::<Arc<[_]>>();

        Ok(TyDef {
            size: offset + compute_padding_for_align(offset, align),
            align,
            kind: TyKind::Aggregate(AggregateTy { fields }),
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
