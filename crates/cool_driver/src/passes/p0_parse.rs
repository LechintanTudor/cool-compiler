use crate::ParsedCrate;
use cool_lexer::{Symbol, TokenStream};
use cool_parser::{DeclKind, Item, Parser};
use cool_resolve::{Mutability, ResolveContext, TyConfig};
use std::fs;

pub fn p0_parse(name: &str, path: &str) -> anyhow::Result<ParsedCrate> {
    let source = fs::read_to_string(path)?;
    let token_stream = TokenStream::new(&source);
    let mut parser = Parser::new(token_stream);

    let source_file = parser.parse_source_file()?;
    let mut resolve = ResolveContext::new_leak(TyConfig { ptr_size: 8 });
    let root_id = resolve.add_root_module(Symbol::insert(name))?;

    for decl in source_file.decls.iter() {
        match &decl.kind {
            DeclKind::Item(item_decl) => {
                match &item_decl.item {
                    Item::Module(_module_item) => {
                        let _item_id = resolve.add_module(
                            root_id,
                            decl.is_exported,
                            item_decl.ident.symbol,
                        )?;
                    }
                    Item::Fn(_fn_item) => {
                        let _item_id = resolve.add_global_binding(
                            root_id,
                            decl.is_exported,
                            Mutability::Const,
                            item_decl.ident.symbol,
                        )?;
                    }
                    item => todo!("{:#?}", item),
                }
            }
            DeclKind::Use(_use_decl) => {
                // Empty
            }
        }
    }

    todo!()
}
