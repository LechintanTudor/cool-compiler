use crate::TyId;
use cool_lexer::Symbol;

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub offset: u64,
    pub symbol: Symbol,
    pub ty: TyId,
}

#[derive(Clone, Debug)]
pub struct AggregateTy {
    fields: Vec<Field>,
}

impl AggregateTy {
    #[inline]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}
