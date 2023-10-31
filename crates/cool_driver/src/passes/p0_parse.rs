use crate::{CompileResult, ModulePaths, ParsedCrate, SourceFile};
use cool_lexer::Symbol;
use cool_parser::{DeclKind, Item, Module, ModuleKind};
use cool_resolve::{ModuleId, ResolveContext, TyConfig};
use std::collections::VecDeque;
use std::path::Path;

pub fn p0_parse(name: &str, path: &Path) -> CompileResult<(ParsedCrate, ResolveContext<'static>)> {
    let mut context = ResolveContext::new_leak(TyConfig { ptr_size: 8 });

    let root_id = context.add_root_module(Symbol::insert(name))?;
    let root_paths = ModulePaths::for_root(path)?;

    let mut file_module_queue = VecDeque::<(ModuleId, ModulePaths)>::new();
    file_module_queue.push_back((root_id, root_paths));

    let mut parsed_crate = ParsedCrate::default();

    while let Some((module_id, module_paths)) = file_module_queue.pop_front() {
        let source_file = SourceFile::from_paths(module_paths)?;
        let module = cool_parser::parse_module(&source_file.source)?;

        let mut modules = VecDeque::<(ModuleId, Module)>::new();
        modules.push_back((module_id, module));

        while let Some((module_id, module)) = modules.pop_front() {
            for decl in module.decls {
                match decl.kind {
                    DeclKind::Item(item_decl) => {
                        match item_decl.item {
                            Item::Module(module) => {
                                let module_id = context.add_module(
                                    module_id,
                                    decl.is_exported,
                                    item_decl.ident.symbol,
                                )?;

                                match module.kind {
                                    ModuleKind::Extern => {
                                        let module_paths = ModulePaths::for_child(
                                            &source_file.paths.child_dir,
                                            item_decl.ident.symbol.as_str(),
                                        )?;

                                        file_module_queue.push_back((module_id, module_paths));
                                    }
                                    ModuleKind::Inline(module) => {
                                        modules.push_back((module_id, module));
                                    }
                                }
                            }
                            item => println!("{:#?}", item),
                        }
                    }
                    decl => println!("{:#?}", decl),
                }
            }
        }

        parsed_crate.files.push(source_file);
    }

    Ok((parsed_crate, context))
}
