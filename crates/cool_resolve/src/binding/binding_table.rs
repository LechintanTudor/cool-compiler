use crate::item::ItemId;
use crate::ty::TyId;
use cool_arena::id_newtype;
use cool_collections::SmallVecMap;
use cool_lexer::symbols::Symbol;

id_newtype!(FrameId);
id_newtype!(BindingId);

#[derive(Clone, Debug)]
pub struct Frame {
    pub module_id: ItemId,
    pub parent_id: Option<FrameId>,
    pub symbols: SmallVecMap<Symbol, BindingId, 2>,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub is_mutable: bool,
    pub ty_id: Option<TyId>,
}

#[derive(Clone, Debug)]
pub struct BindingTable {
    frames: Vec<Frame>,
    bindings: Vec<Binding>,
}

impl BindingTable {
    pub fn add_frame(&mut self, module_id: ItemId, parent_id: Option<FrameId>) -> FrameId {
        let frame_id = FrameId::new_unwrap(self.frames.len() as u32);

        self.frames.push(Frame {
            module_id,
            parent_id,
            symbols: Default::default(),
        });

        frame_id
    }

    #[must_use]
    pub fn add_binding(
        &mut self,
        frame_id: FrameId,
        symbol: Symbol,
        is_mutable: bool,
        ty_id: Option<TyId>,
    ) -> Option<BindingId> {
        let symbols = &mut self.frames[frame_id.as_usize()].symbols;

        if symbols.contains_key(&symbol) {
            return None;
        }

        let binding_id = BindingId::new_unwrap(self.bindings.len() as u32);

        self.bindings.push(Binding { is_mutable, ty_id });
        symbols.insert_unchecked(symbol, binding_id);

        Some(binding_id)
    }

    pub fn set_binding_ty_id(&mut self, frame_id: FrameId, symbol: Symbol, ty_id: TyId) {
        let binding_id = self.frames[frame_id.as_usize()].symbols[&symbol];
        let ty_id_ref = &mut self.bindings[binding_id.as_usize()].ty_id;

        // TODO: Check that ty_id is unassigned
        *ty_id_ref = Some(ty_id);
    }

    pub fn get_binding(&self, frame_id: FrameId, symbol: Symbol) -> Option<&Binding> {
        let binding_id = self.frames[frame_id.as_usize()].symbols.get(&symbol)?;
        Some(&self.bindings[binding_id.as_usize()])
    }

    #[inline]
    pub fn get_binding_id(&self, frame_id: FrameId, symbol: Symbol) -> Option<BindingId> {
        self.frames[frame_id.as_usize()]
            .symbols
            .get(&symbol)
            .copied()
    }

    pub fn get_binding_by_id(&self, binding_id: BindingId) -> &Binding {
        &self.bindings[binding_id.as_usize()]
    }

    #[inline]
    pub fn iter_frames(&self) -> impl Iterator<Item = &Frame> + '_ {
        self.frames.iter()
    }
}

impl Default for BindingTable {
    fn default() -> Self {
        Self {
            frames: vec![Frame {
                module_id: ItemId::dummy(),
                parent_id: None,
                symbols: Default::default(),
            }],
            bindings: vec![Binding {
                is_mutable: false,
                ty_id: None,
            }],
        }
    }
}
