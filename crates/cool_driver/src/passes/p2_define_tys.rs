use crate::{CompileError, CompileErrorBundle, CompileResult, DefineItem, Package};
use cool_ast::AstGenerator;
use cool_lexer::Symbol;
use cool_resolve::{DefineError, DefineErrorKind, ResolveContext, TyId};
use cool_span::Section;
use std::collections::VecDeque;

pub fn p2_define_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut ast = AstGenerator::new(resolve);
    let mut errors = Vec::<CompileError>::new();

    define_aliases(package, &mut ast, &mut errors);
    define_enums(package, &mut ast, &mut errors);
    define_structs(package, &mut ast, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}

fn define_aliases(package: &Package, ast: &mut AstGenerator, errors: &mut Vec<CompileError>) {
    let mut aliases = package.aliases.iter().collect::<VecDeque<_>>();
    let mut resolve_fail_count = 0;

    while let Some(item) = aliases.pop_front() {
        match ast.resolve_ty(item.module_id, &item.item.ty) {
            Ok(resolved_ty_id) => {
                ast.resolve.define_alias(item.item_id, resolved_ty_id);
                resolve_fail_count = 0;
            }
            Err(_) => {
                aliases.push_back(item);

                resolve_fail_count += 1;
                if resolve_fail_count >= aliases.len() {
                    break;
                }
            }
        }
    }

    report_undefinable_tys(ast, errors, aliases);
}

fn define_enums(package: &Package, ast: &mut AstGenerator, errors: &mut Vec<CompileError>) {
    for item in package.enums.iter() {
        let storage = item
            .item
            .storage
            .as_ref()
            .map(|storage| ast.resolve_ty(item.module_id, &storage.ty))
            .transpose();

        let storage = match storage {
            Ok(storage) => storage,
            Err(error) => {
                errors.push(error.into());
                continue;
            }
        };

        let variants = item.item.variants.iter().map(|ident| ident.symbol);

        if let Err(error) = ast.resolve.define_enum(item.item_id, storage, variants) {
            errors.push(CompileError::Define {
                span: item.span,
                error,
            });
            continue;
        }
    }
}

fn define_structs(package: &Package, ast: &mut AstGenerator, errors: &mut Vec<CompileError>) {
    let mut structs = package.structs.iter().collect::<VecDeque<_>>();
    let mut resolve_fail_count = 0;

    'struct_loop: while let Some(item) = structs.pop_front() {
        let fields = {
            let mut fields = Vec::<(Symbol, TyId)>::new();

            for field in item.item.fields.iter() {
                let ty_id = match ast.resolve_ty(item.module_id, &field.ty) {
                    Ok(ty_id) => ty_id,
                    Err(error) => {
                        errors.push(error.into());
                        continue 'struct_loop;
                    }
                };

                fields.push((field.ident.symbol, ty_id));
            }

            fields
        };

        match ast.resolve.define_struct(item.item_id, fields) {
            Ok(true) => resolve_fail_count = 0,
            Ok(false) => {
                structs.push_back(item);

                resolve_fail_count += 1;
                if resolve_fail_count >= structs.len() {
                    break;
                }
            }
            Err(error) => {
                errors.push(CompileError::Define {
                    span: item.item.span(),
                    error,
                });
            }
        }
    }

    report_undefinable_tys(ast, errors, structs);
}

fn report_undefinable_tys<'a, I>(
    ast: &mut AstGenerator,
    errors: &mut Vec<CompileError>,
    items: impl IntoIterator<Item = &'a DefineItem<I>>,
) where
    I: 'a,
{
    items.into_iter().for_each(|item| {
        errors.push(CompileError::Define {
            span: item.span,
            error: DefineError {
                path: ast.resolve.get_path_by_item_id(item.item_id).to_path_buf(),
                kind: DefineErrorKind::TypeCannotBeDefined,
            },
        });
    })
}
