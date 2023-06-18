use crate::{CompileError, CompileErrorBundle, CompileResult, Package};
use cool_ast::AstGenerator;
use cool_resolve::{DefineError, DefineErrorKind, Field, ResolveContext};
use cool_span::Section;
use std::collections::VecDeque;

pub fn p2_define_tys(package: &Package, resolve: &mut ResolveContext) -> CompileResult<()> {
    let mut ast = AstGenerator::new(resolve);
    let mut errors = Vec::<CompileError>::new();

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

    let mut structs = package.structs.iter().collect::<VecDeque<_>>();
    let mut resolve_fail_count = 0;

    'struct_loop: while let Some(item) = structs.pop_front() {
        let fields = {
            let mut fields = Vec::<Field>::new();

            for field in item.item.fields.iter() {
                let ty_id = match ast.resolve_ty(item.module_id, &field.ty) {
                    Ok(ty_id) => ty_id,
                    Err(error) => {
                        errors.push(error.into());
                        continue 'struct_loop;
                    }
                };

                fields.push(Field {
                    offset: 0,
                    symbol: field.ident.symbol,
                    ty_id,
                });
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

    let undefinable_tys = {
        let undefinable_aliases = aliases.iter().map(|item| (item.span, item.item_id));
        let undefinable_structs = structs.iter().map(|item| (item.span, item.item_id));
        undefinable_aliases.chain(undefinable_structs)
    };

    undefinable_tys.for_each(|(span, item_id)| {
        errors.push(CompileError::Define {
            span,
            error: DefineError {
                path: ast.resolve.get_path_by_item_id(item_id).to_path_buf(),
                kind: DefineErrorKind::TypeCannotBeDefined,
            },
        });
    });

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CompileErrorBundle { errors })
    }
}
