use crate::{ItemResult, ModuleId, ResolveContext};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_import(
        &mut self,
        _module_id: ModuleId,
        _is_exported: bool,
        _path: &[Symbol],
    ) -> ItemResult<()> {
        todo!()
    }
}
