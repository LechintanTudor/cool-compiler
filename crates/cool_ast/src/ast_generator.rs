use cool_resolve::item::{ItemPath, ItemTable};
use cool_resolve::ty::{TyId, TyTable};

pub struct AstGenerator<'a> {
    pub items: &'a ItemTable,
    pub tys: &'a mut TyTable,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(items: &'a ItemTable, tys: &'a mut TyTable) -> Self {
        Self { items, tys }
    }

    pub fn get_ty_id_by_path<'b, P>(&self, path: P) -> Option<TyId>
    where
        P: Into<ItemPath<'b>>,
    {
        let item_id = self.items.get_id_by_path(path)?;
        self.tys.get_ty_id_by_item_id(item_id)
    }
}
