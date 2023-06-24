use crate::{ResolveTy, TyDef};
use cool_arena::InternedValue;
use cool_lexer::Symbol;
use derive_more::{Deref, Display, From};

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Deref, Display, Debug)]
#[deref(forward)]
#[display(fmt = "{}", "_0.shape")]
pub struct TyId(InternedValue<'static, ResolveTy>);

impl TyId {
    #[must_use]
    pub fn define_struct<F>(&self, fields: F) -> bool
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        assert!(self.shape.is_struct());

        let Ok(def) = TyDef::aggregate(fields) else {
            return false;
        };

        let mut def_slot = match &self.def {
            TyDef::Deferred(def) => def.lock().unwrap(),
            _ => return true,
        };

        *def_slot = Some(def);
        true
    }
}
