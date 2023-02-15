use crate::item::{FnItem, ModuleItem};

#[derive(Clone, Debug)]
pub enum Item {
    Module(ModuleItem),
    Fn(FnItem),
}

impl From<ModuleItem> for Item {
    fn from(module_item: ModuleItem) -> Self {
        Self::Module(module_item)
    }
}

impl From<FnItem> for Item {
    fn from(fn_item: FnItem) -> Self {
        Self::Fn(fn_item)
    }
}
