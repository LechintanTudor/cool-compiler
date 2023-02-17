use crate::item::{FnItem, ModuleItem};
use crate::ParseTree;
use cool_span::Span;

#[derive(Clone, Debug)]
pub enum Item {
    Module(ModuleItem),
    Fn(FnItem),
}

impl ParseTree for Item {
    fn span(&self) -> Span {
        match self {
            Self::Module(module) => module.span(),
            Self::Fn(function) => function.span(),
        }
    }
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
