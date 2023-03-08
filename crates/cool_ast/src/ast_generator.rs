use cool_parser::ty::Ty;
use cool_resolve::item::{ItemPathBuf, ItemTable};
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

    pub fn resolve_ty(&mut self, parsed_ty: &Ty) -> Option<TyId> {
        match parsed_ty {
            Ty::Path(path) => {
                let path = path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self.items.get_id_by_path(&path)?;
                self.tys.get_ty_id_by_item_id(item_id)
            }
            Ty::Tuple(tuple_ty) => {
                let mut elems = Vec::<TyId>::new();

                for ty in tuple_ty.elements.iter() {
                    elems.push(self.resolve_ty(ty)?);
                }

                Some(self.tys.mk_tuple(elems))
            }
        }
    }
}
