use crate::resolve::{
    Binding, BindingId, Frame, FrameId, ModuleId, ResolveError, ResolveResult, ResolveTable,
    ScopeId, SymbolKind,
};
use cool_lexer::symbols::Symbol;

impl ResolveTable {
    #[inline]
    pub fn insert_frame(&mut self, parent_id: ScopeId) -> FrameId {
        self.frames.push(Frame::new(parent_id))
    }

    pub fn insert_local_binding(
        &mut self,
        frame_id: FrameId,
        is_mutable: bool,
        symbol: Symbol,
    ) -> ResolveResult<BindingId> {
        let binding_id = self.bindings.push(Binding::new(is_mutable));

        if !self.frames[frame_id]
            .bindings
            .insert_if_not_exists(symbol, binding_id)
        {
            return Err(ResolveError::already_defined(symbol));
        }

        Ok(binding_id)
    }

    pub fn resolve_local(&self, frame_id: FrameId, symbol: Symbol) -> ResolveResult<SymbolKind> {
        let mut scope_id = ScopeId::Frame(frame_id);

        loop {
            match scope_id {
                ScopeId::Frame(frame_id) => {
                    let frame = &self.frames[frame_id];
                    let resolved_symbol = frame
                        .bindings
                        .get(&symbol)
                        .map(|&binding_id| SymbolKind::Binding(binding_id));

                    match resolved_symbol {
                        Some(resolved_symbol) => return Ok(resolved_symbol),
                        None => scope_id = frame.parent_id,
                    }
                }
                ScopeId::Module(module_id) => {
                    let resolved_elem = self.modules[&module_id].elems.get(&symbol);

                    match resolved_elem {
                        Some(resolved_elem) => return Ok(resolved_elem.kind),
                        None => return Err(ResolveError::not_found(symbol)),
                    }
                }
            }
        }
    }

    pub fn resolve_local_access(
        &self,
        parent_id: ModuleId,
        source_id: ModuleId,
        symbol: Symbol,
    ) -> ResolveResult<SymbolKind> {
        let parent_module = &self.modules[&parent_id];
        let source_module = &self.modules[&source_id];

        let resolved_elem = source_module
            .elems
            .get(&symbol)
            .ok_or(ResolveError::not_found(symbol))?;

        if !resolved_elem.is_exported && !parent_module.path.starts_with(&source_module.path) {
            return Err(ResolveError::private(symbol));
        }

        Ok(resolved_elem.kind)
    }
}
