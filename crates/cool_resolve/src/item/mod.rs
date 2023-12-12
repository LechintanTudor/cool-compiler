mod crates;
mod module;
mod path;

pub use self::crates::*;
pub use self::module::*;
pub use self::path::*;

use crate::TyId;
use cool_collections::define_index_newtype;
use derive_more::From;

define_index_newtype!(ItemId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum Item {
    Crate(CrateId),
    Module(ModuleId),
    Ty(TyId),
}
