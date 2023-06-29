use crate::{
    compute_padding_for_align, TyContext, TyDef, TyError, TyErrorKind, TyId, TyKind, TyResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TaggedUnionKind {
    Basic,
    NullablePtr,
}

#[derive(Clone, Debug)]
pub struct TaggedUnionTy {
    pub dominant_ty_id: TyId,
    pub padding_before_index: u64,
    pub kind: TaggedUnionKind,
}

impl TyContext {
    pub(crate) fn mk_tagged_union_ty_def<V>(&mut self, ty_id: TyId, variants: V) -> TyResult<TyDef>
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

        let kind = TaggedUnionKind::Basic;
        let align = dominant_ty_def.align;

        let (size, padding_before_index) = match kind {
            TaggedUnionKind::Basic => {
                let padding_before_index =
                    compute_padding_for_align(align + largest_size, self.primitives.i8_align);

                let mut size = largest_size + padding_before_index + 1;
                size = size + compute_padding_for_align(size, align);

                (size, padding_before_index)
            }
            TaggedUnionKind::NullablePtr => (dominant_ty_def.size, 0),
        };

        Ok(TyDef {
            size,
            align,
            kind: TyKind::from(TaggedUnionTy {
                dominant_ty_id,
                padding_before_index,
                kind,
            }),
        })
    }
}
