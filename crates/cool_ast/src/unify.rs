use crate::AstGenerator;

pub trait Unify {
    fn unify(&self, gen: &mut AstGenerator);
}
