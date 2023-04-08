use crate::item::item_decl::ItemDeclAst;
use crate::{AstGenerator, ConstItemAst, ItemAst, ResolveAst, SemanticResult, TyMismatch};
use cool_parser::{ConstItem, Expr, Item, ModuleContent};
use cool_resolve::{tys, ModuleId, TyId};

#[derive(Clone, Debug)]
pub struct ModuleItemAst {
    pub decls: Vec<ItemDeclAst>,
}

impl ResolveAst for ModuleItemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        tys::MODULE
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: tys::MODULE,
                expected_ty,
            })?;

        for decl in self.decls.iter() {
            decl.resolve(ast, tys::INFERRED)?;
        }

        Ok(tys::MODULE)
    }
}

impl AstGenerator<'_> {
    pub fn gen_module(&mut self, module_id: ModuleId, module: &ModuleContent) -> ModuleItemAst {
        let mut decls = Vec::<ItemDeclAst>::new();

        for decl in module
            .decls
            .iter()
            .flat_map(|decl| decl.kind.as_item_decl())
        {
            let explicit_ty_id = match &decl.ty {
                Some(ty) => self.resolve_parsed_ty(module_id.into(), ty).unwrap(),
                None => tys::INFERRED,
            };

            let item: ItemAst = match &decl.item {
                Item::Const(ConstItem {
                    expr: Expr::Fn(fn_item),
                }) => ConstItemAst {
                    expr: self.gen_fn(module_id, fn_item).into(),
                }
                .into(),
                _ => todo!(),
            };

            decls.push(ItemDeclAst {
                symbol: decl.ident.symbol,
                explicit_ty_id,
                item,
            });
        }

        ModuleItemAst { decls }
    }
}
