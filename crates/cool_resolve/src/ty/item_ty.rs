use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemTy {
    Module,
    Ty,
}

impl fmt::Display for ItemTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::Module => "module",
            Self::Ty => "type",
        };

        write!(f, "{display_str}")
    }
}
