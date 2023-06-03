use crate::{ItemId, TyId};
use cool_lexer::Symbol;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Eq, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty_id: TyId,
}

impl PartialEq for Field {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.ty_id == other.ty_id
    }
}

impl Hash for Field {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.symbol.hash(state);
        self.ty_id.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AggregateKind {
    Struct(ItemId),
    Tuple,
    Slice,
}

#[derive(Clone, Eq, Debug)]
pub struct AggregateTy {
    pub kind: AggregateKind,
    pub fields: Vec<Field>,
}

impl AggregateTy {
    #[inline]
    pub fn get_field_ty_id(&self, symbol: Symbol) -> Option<TyId> {
        self.fields
            .iter()
            .find(|field| field.symbol == symbol)
            .map(|field| field.ty_id)
    }
}

impl PartialEq for AggregateTy {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (AggregateKind::Struct(i1), AggregateKind::Struct(i2)) => i1 == i2,
            _ => self.fields == other.fields,
        }
    }
}

impl Hash for AggregateTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self.kind {
            AggregateKind::Struct(item_id) => item_id.hash(state),
            _ => {
                self.fields.hash(state);
            }
        }
    }
}
