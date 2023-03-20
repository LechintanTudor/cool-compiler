use crate::item::{ItemPath, ItemPathBuf};
use cool_arena::SliceArena;
use cool_collections::{id_newtype, IdIndexedVec, SmallVecMap};
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

id_newtype!(ItemId);
id_newtype!(ModuleId);
id_newtype!(FrameId);
id_newtype!(BindingId);

impl ModuleId {
    #[inline]
    pub fn as_item_id(&self) -> ItemId {
        ItemId(self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub path: ItemPathBuf,
    pub types: FxHashMap<Symbol, ModuleItem>,
    pub values: FxHashMap<Symbol, ModuleBinding>,
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleItem {
    pub is_exported: bool,
    pub item_id: ItemId,
}

#[derive(Clone, Copy, Debug)]
pub struct ModuleBinding {
    pub is_exported: bool,
    pub binding_id: BindingId,
}

#[derive(Clone, Copy, Debug)]
pub enum ParentId {
    Frame(FrameId),
    Module(ModuleId),
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent_id: ParentId,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub is_mutable: bool,
}

pub struct NameTable {
    item_paths: SliceArena<ItemId, Symbol>,
    modules: FxHashMap<ModuleId, Module>,
    frames: IdIndexedVec<FrameId, Frame>,
    bindings: IdIndexedVec<BindingId, Binding>,
}

impl NameTable {
    pub fn add_module(&mut self, parent_id: ModuleId, is_exported: bool, symbol: Symbol) {
        todo!()
    }

    pub fn add_type(&mut self, module_id: ModuleId, is_exported: bool, symbol: Symbol) -> ItemId {
        todo!()
    }

    pub fn add_use<'a, P>(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        item_path: P,
        symbol: Option<Symbol>,
    ) where
        P: Into<ItemPath<'a>>,
    {
    }

    pub fn add_frame(&mut self, parent_id: ParentId) -> FrameId {
        self.frames.push(Frame {
            parent_id,
            bindings: Default::default(),
        })
    }

    pub fn add_value(&mut self, parent_id: ParentId, is_mutable: bool, symbol: Symbol) -> ItemId {
        todo!()
    }

    pub fn resolve_ty(&self, frame_id: FrameId, symbol: Symbol) {
        todo!()
    }

    pub fn resolve_binding(&self, frame_id: FrameId, symbol: Symbol) {
        todo!()
    }
}

// impl Default for NameTable {
//     fn default() -> Self {
//         Self {
//             frames: vec![
//                 Frame {
//                     parent_id: FrameId::dummy(),
//                 }
//             ],
//             bindings: vec![],
//         }
//     }
// }
