use crate::TyId;
use cool_lexer::Symbol;
use std::cmp::Reverse;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

impl PartialEq for Field {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.symbol, self.ty_id) == (other.symbol, other.ty_id)
    }
}

impl Eq for Field {}

impl Hash for Field {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.symbol.hash(state);
        self.ty_id.hash(state);
    }
}

pub fn resolve_fields_size_align(fields: &mut [Field]) -> (u64, u64) {
    fields.sort_by_key(|field| Reverse(field.ty_id.get_align()));

    let mut offset = 0;
    let mut align = 1;

    for field in fields.iter_mut() {
        let field_align = field.ty_id.get_align();
        let field_size = field.ty_id.get_size();

        field.offset = offset;
        offset += compute_padding_for_align(offset, field_align) + field_size;
        align = align.max(field_align);
    }

    let size = offset + compute_padding_for_align(offset, align);
    (size, align)
}

fn compute_padding_for_align(offset: u64, align: u64) -> u64 {
    let misalign = offset % align;

    if misalign > 0 {
        align - misalign
    } else {
        0
    }
}
