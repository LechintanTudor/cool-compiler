use crate::TyId;
use cool_lexer::symbols::Symbol;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mutability {
    Const,
    Immutable,
    Mutable,
}

impl Mutability {
    #[inline]
    pub fn local(is_mutable: bool) -> Self {
        if is_mutable {
            Self::Mutable
        } else {
            Self::Immutable
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub symbol: Symbol,
    pub mutability: Mutability,
    pub ty_id: TyId,
}

impl Binding {
    #[inline]
    pub fn is_mutable(&self) -> bool {
        self.mutability == Mutability::Mutable
    }
}
