use crate::context::{FrameId, ItemKind, ModuleId, ResolveContext, ResolveError, ResolveResult};
use crate::ScopeId;
use cool_lexer::symbols::Symbol;

impl ResolveContext {
    pub fn resolve_local(&self, frame_id: FrameId, symbol: Symbol) -> ResolveResult<ItemKind> {
        let mut scope_id = ScopeId::Frame(frame_id);

        loop {
            match scope_id {
                ScopeId::Frame(frame_id) => {
                    let frame = &self.frames[frame_id];
                    let resolved_symbol = frame
                        .bindings
                        .get(&symbol)
                        .map(|&binding_id| ItemKind::Binding(binding_id));

                    match resolved_symbol {
                        Some(resolved_symbol) => return Ok(resolved_symbol),
                        None => scope_id = frame.parent_id,
                    }
                }
                ScopeId::Module(module_id) => {
                    let resolved_elem = self.modules[module_id].elems.get(&symbol);

                    match resolved_elem {
                        Some(resolved_elem) => return Ok(self.items[resolved_elem.item_id]),
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
    ) -> ResolveResult<ItemKind> {
        let parent_module = &self.modules[parent_id];
        let source_module = &self.modules[source_id];

        let resolved_elem = source_module
            .elems
            .get(&symbol)
            .ok_or(ResolveError::not_found(symbol))?;

        if !resolved_elem.is_exported && !parent_module.path.starts_with(&source_module.path) {
            return Err(ResolveError::private(symbol));
        }

        Ok(self.items[resolved_elem.item_id])
    }
}
