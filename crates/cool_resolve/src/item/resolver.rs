use crate::item::{ItemError, ItemErrorKind, ItemId, ItemPath, ItemPathBuf, ItemTable};
use cool_lexer::symbols::sym;

pub fn resolve_import(
    items: &ItemTable,
    module_id: ItemId,
    import_path: ItemPath,
) -> Result<Option<ItemId>, ItemError> {
    let module_path = items.get_path_by_id(module_id).unwrap();

    let mut resolved_path = if import_path.starts_with_crate() {
        ItemPathBuf::from(module_path.first())
    } else if import_path.starts_with_self_or_super() {
        module_path.to_path_buf()
    } else {
        todo!()
    };

    for symbol in import_path.as_symbol_slice() {
        resolved_path = match *symbol {
            sym::KW_SELF | sym::KW_CRATE => continue,
            sym::KW_SUPER => resolved_path.pop(), // TODO: Return error if resolved path gets empty,
            symbol => resolved_path.append(symbol),
        };
    }

    let Some((final_symbol, module_symbols)) = resolved_path.as_symbol_slice().split_last() else {
        panic!("resolved path is empty");
    };

    let mut current_module = match items.get_module_by_path(&module_symbols[..1]) {
        Ok(Some(module)) => module,
        Ok(None) => return Ok(None),
        Err(kind) => {
            return Err(ItemError {
                kind,
                module_path: module_path.to_path_buf(),
                symbol_path: import_path.to_path_buf(),
            })
        }
    };

    for symbol in &module_symbols[1..] {
        let Some(module_item) = current_module.items.get(symbol) else {
            return Ok(None);
        };

        if !module_item.is_exported && !module_path.starts_with(&current_module.path) {
            return Err(ItemError {
                kind: ItemErrorKind::SymbolIsUnreachable,
                module_path: module_path.to_path_buf(),
                symbol_path: import_path.to_path_buf(),
            });
        };

        let next_module = match items.get_module_by_id(module_item.item_id) {
            Ok(Some(module)) => module,
            Ok(None) => return Ok(None),
            Err(kind) => {
                return Err(ItemError {
                    kind,
                    module_path: module_path.to_path_buf(),
                    symbol_path: import_path.to_path_buf(),
                })
            }
        };

        current_module = next_module;
    }

    let item_id = {
        let Some(module_item) = current_module.items.get(final_symbol) else {
            return Ok(None);
        };

        if !module_item.is_exported && !module_path.starts_with(&current_module.path) {
            return Err(ItemError {
                kind: ItemErrorKind::SymbolIsUnreachable,
                module_path: module_path.to_path_buf(),
                symbol_path: import_path.to_path_buf(),
            });
        }

        module_item.item_id
    };

    Ok(Some(item_id))
}
