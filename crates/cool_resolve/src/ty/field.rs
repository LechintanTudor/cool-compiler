use crate::TyId;
use cool_lexer::Symbol;
use std::cmp::Reverse;

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

pub fn resolve_fields_size_align(fields: &mut [Field]) -> Option<(u64, u64)> {
    fields.sort_by_key(|field| Reverse(field.ty_id.def.get_align()));

    let mut offset = 0;
    let mut align = 1;

    for field in fields.iter_mut() {
        let (field_size, field_align) = field.ty_id.def.try_get_size_align()?;

        field.offset = offset;
        offset += compute_padding_for_align(offset, field_align) + field_size;
        align = align.max(field_align);
    }

    let size = offset + compute_padding_for_align(offset, align);
    Some((size, align))
}

fn compute_padding_for_align(offset: u64, align: u64) -> u64 {
    let misalign = offset % align;

    if misalign > 0 {
        align - misalign
    } else {
        0
    }
}
