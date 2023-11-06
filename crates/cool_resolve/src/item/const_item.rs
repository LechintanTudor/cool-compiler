use crate::{ItemId, ModuleId, ResolveContext, ResolveResult, TyId};
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use std::ops::Index;

define_index_newtype!(ConstId);

#[derive(Clone, Debug)]
pub struct ConstItem {
    pub ty_id: TyId,
    pub value: ConstItemValue,
}

#[derive(Clone, Debug)]
pub enum ConstItemValue {
    Undefined,
    Fn,
    Int(u128),
}

impl ConstItemValue {
    #[inline]
    #[must_use]
    pub fn try_as_int(&self) -> Option<u128> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }
}

impl ResolveContext<'_> {
    pub fn add_const(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        symbol: Symbol,
        ty_id: TyId,
        value: ConstItemValue,
    ) -> ResolveResult<ItemId> {
        let item_id = self.add_path(module_id, symbol)?;
        let const_id = self.consts.push(ConstItem { ty_id, value });
        self.add_item(module_id, is_exported, symbol, item_id, const_id);
        Ok(item_id)
    }

    pub fn update_const(&mut self, item_id: ItemId, ty_id: TyId, value: ConstItemValue) {
        let const_id = self.items[&item_id].try_as_const().unwrap();
        self.consts[const_id] = ConstItem { ty_id, value };
    }
}

impl Index<ConstId> for ResolveContext<'_> {
    type Output = ConstItem;

    #[inline]
    #[must_use]
    fn index(&self, const_id: ConstId) -> &Self::Output {
        &self.consts[const_id]
    }
}
