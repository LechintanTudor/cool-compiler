use crate::context::{FrameId, ItemKind, ModuleId, ResolveContext, ResolveError, ResolveResult};
use crate::{ResolveErrorKind, Scope};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn resolve_local(&self, frame_id: FrameId, symbol: Symbol) -> ResolveResult<ItemKind> {
        let mut scope = Scope::Frame(frame_id);

        loop {
            match scope {
                Scope::Frame(frame_id) => {
                    let frame = &self.frames[frame_id];
                    let resolved_symbol = frame
                        .bindings
                        .get(&symbol)
                        .map(|&binding_id| ItemKind::Binding(binding_id));

                    match resolved_symbol {
                        Some(resolved_symbol) => return Ok(resolved_symbol),
                        None => scope = frame.parent,
                    }
                }
                Scope::Module(module_id) => {
                    let resolved_elem = self.modules[module_id].elems.get(&symbol);

                    match resolved_elem {
                        Some(resolved_elem) => return Ok(self.items[&resolved_elem.item_id]),
                        None => {
                            return Err(ResolveError {
                                symbol,
                                kind: ResolveErrorKind::SymbolNotFound,
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn resolve_local_access(
        &self,
        parent: ModuleId,
        source_id: ModuleId,
        symbol: Symbol,
    ) -> ResolveResult<ItemKind> {
        let parent_module = &self.modules[parent];
        let source_module = &self.modules[source_id];

        let Some(elem) = source_module.elems.get(&symbol) else {
            return Err(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolNotFound,
            });
        };

        if !elem.is_exported && !source_module.item_id.is_child_of(parent_module.item_id) {
            return Err(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolNotPublic,
            });
        }

        Ok(self.items[&elem.item_id])
    }
}
