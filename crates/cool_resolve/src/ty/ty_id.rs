use crate::TyShape;
use cool_arena::InternedValue;
use derive_more::{Deref, Display, From};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, Display, Debug)]
#[deref(forward)]
pub struct TyId(InternedValue<'static, TyShape>);
