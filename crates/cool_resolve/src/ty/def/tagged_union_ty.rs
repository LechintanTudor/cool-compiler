use crate::{
    compute_padding_for_align, TyContext, TyDef, TyError, TyErrorKind, TyId, TyKind, TyResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TaggedUnionKind {
    Basic { padding_before_index: u64 },
    NullablePtr,
}

#[derive(Clone, Debug)]
pub struct TaggedUnionTy {
    pub dominant_ty_id: TyId,
    pub kind: TaggedUnionKind,
}

impl TyContext {
    pub(crate) fn mk_tagged_union_ty_def<V>(&self, ty_id: TyId, variants: V) -> TyResult<TyDef>
    where
        V: IntoIterator<Item = TyId>,
    {
        let variants = variants
            .into_iter()
            .map(|ty_id| self.get_def(ty_id).map(|def| (ty_id, def)))
            .collect::<Option<Vec<_>>>()
            .ok_or(TyError {
                ty_id,
                kind: TyErrorKind::CannotBeDefined,
            })?;

        if let Some(ty_def) = self.try_mk_nullable_ptr_ty_def(&variants) {
            return Ok(ty_def);
        }

        let mut variants = variants.into_iter();
        let (mut dominant_ty_id, mut dominant_ty_def, mut largest_size) = variants
            .next()
            .map(|(ty_id, def)| (ty_id, def, def.size))
            .unwrap();

        for (ty_id, def) in variants {
            if def.size > largest_size {
                largest_size = def.size;
            }

            if def.align > dominant_ty_def.align {
                dominant_ty_id = ty_id;
                dominant_ty_def = def;
            }
        }

        let align = dominant_ty_def.align;

        let padding_before_index =
            compute_padding_for_align(align + largest_size, self.primitives.i8_align);

        let mut size = largest_size + padding_before_index + 1;
        size = size + compute_padding_for_align(size, align);

        Ok(TyDef {
            size,
            align,
            kind: TyKind::from(TaggedUnionTy {
                dominant_ty_id,
                kind: TaggedUnionKind::Basic {
                    padding_before_index,
                },
            }),
        })
    }

    pub(crate) fn try_mk_nullable_ptr_ty_def(&self, variants: &[(TyId, &TyDef)]) -> Option<TyDef> {
        let (&dominant_ty_id, &dominant_ty_def) = match variants {
            [(first_ty_id, first_ty_def), (second_ty_id, second_ty_def)] => {
                if first_ty_id.get_value().is_ptr_like() && second_ty_def.is_zero_sized() {
                    (first_ty_id, first_ty_def)
                } else if second_ty_id.get_value().is_ptr_like() && first_ty_def.is_zero_sized() {
                    (second_ty_id, second_ty_def)
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        Some(TyDef {
            size: dominant_ty_def.size,
            align: dominant_ty_def.align,
            kind: TyKind::from(TaggedUnionTy {
                dominant_ty_id,
                kind: TaggedUnionKind::NullablePtr,
            }),
        })
    }
}
