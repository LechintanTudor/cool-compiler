use crate::{tys, ItemId, ModuleId, ResolveContext, ResolveResult, TyId};
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;

define_index_newtype!(BindingId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Binding {
    pub mutability: Mutability,
    pub ty: TyId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mutability {
    Const,
    Immutable,
    Mutable,
}

impl ResolveContext {
    pub fn add_global_binding(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
    ) -> ResolveResult<ItemId> {
        self.add_item(module_id, is_exported, symbol, |context| {
            context.bindings.push(Binding {
                mutability: Mutability::Const,
                ty: tys::infer,
            })
        })
    }

    #[inline]
    pub fn define_global_binding(&mut self, binding_id: BindingId, ty_id: TyId) {
        let binding = &mut self.bindings[binding_id];
        assert_eq!(binding.ty, tys::infer);
        binding.ty = ty_id;
    }
}
