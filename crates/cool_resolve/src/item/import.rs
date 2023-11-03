use crate::{ModuleElem, ModuleId, ResolveContext, ResolveError, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext<'_> {
    pub fn add_import(
        &mut self,
        module_id: ModuleId,
        is_exported: bool,
        path: &[Symbol],
        alias: Option<Symbol>,
    ) -> ResolveResult<()> {
        let item_id = self.resolve_path(module_id, path)?;
        let symbol = alias.unwrap_or(*path.last().unwrap());
        let module = &mut self.modules[module_id];

        if module.elems.contains_key(&symbol) {
            return Err(ResolveError::SymbolAlreadyExists {
                symbol: *path.last().unwrap(),
            });
        }

        module.elems.insert(
            symbol,
            ModuleElem {
                is_exported,
                item_id,
            },
        );

        Ok(())
    }
}
