use crate::item::itm;
use cool_arena::handle_newtype;

handle_newtype!(ItemId; Debug);

impl ItemId {
    #[inline]
    pub const fn is_builtin(&self) -> bool {
        self.0.index() <= itm::F64.0.index()
    }
}
