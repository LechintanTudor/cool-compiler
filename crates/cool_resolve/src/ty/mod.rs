pub mod tys {
    pub use crate::consts::tys::*;
}

mod ty_kind;
mod ty_table;

pub use self::ty_kind::*;
pub use self::ty_table::*;
