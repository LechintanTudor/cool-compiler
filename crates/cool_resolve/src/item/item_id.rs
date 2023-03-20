use crate::item::itm;
use cool_collections::id_newtype;

id_newtype!(ItemId);

impl ItemId {
    #[inline]
    pub const fn is_builtin(&self) -> bool {
        self.index() <= itm::F64.index()
    }
}
