use crate::resolve::{
    BindingId, FrameId, ModuleId, ResolveResult, ResolveTable, ScopeId, SymbolKind,
};
use cool_lexer::symbols::Symbol;

impl ResolveTable {
    pub fn insert_frame(&mut self, _parent_id: ScopeId) -> FrameId {
        todo!()
    }

    pub fn insert_local_binding(
        &mut self,
        _parent_id: FrameId,
        _is_mutable: bool,
        _symbol: Symbol,
    ) -> ResolveResult<BindingId> {
        todo!()
    }

    pub fn resolve_local(&self, _frame_id: FrameId, _symbol: Symbol) -> ResolveResult<SymbolKind> {
        todo!()
    }

    pub fn resolve_local_access(
        &self,
        _parent_id: ModuleId,
        _source_id: ModuleId,
        _symbol: Symbol,
    ) -> ResolveResult<SymbolKind> {
        todo!()
    }
}
