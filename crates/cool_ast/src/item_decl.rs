use crate::{AstGenerator, AstResult, ItemAst, ResolveAst};
use cool_resolve::{tys, ItemId, TyId};

#[derive(Clone, Debug)]
pub struct ItemDeclAst {
    pub explicit_ty_id: TyId,
    pub item_id: ItemId,
    pub item: ItemAst,
}

impl ResolveAst for ItemDeclAst {
    fn resolve_consts(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        debug_assert_eq!(expected_ty, tys::INFERRED);

        match &self.item {
            ItemAst::Module(module_ast) => {
                module_ast.resolve_consts(ast, self.explicit_ty_id)?;
            }
            ItemAst::Const(const_ast) => {
                let ty_id = const_ast.resolve_consts(ast, self.explicit_ty_id)?;
                let binding_id = ast.resolve[self.item_id].as_binding_id().unwrap();
                ast.resolve.set_binding_ty(binding_id, ty_id);
            }
        }

        Ok(tys::UNIT)
    }

    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        debug_assert_eq!(expected_ty, tys::INFERRED);

        let ty_id = self.item.resolve_exprs(ast, self.explicit_ty_id)?;
        let binding_id = ast.resolve[self.item_id].as_binding_id().unwrap();
        ast.resolve.set_binding_ty(binding_id, ty_id);

        Ok(tys::UNIT)
    }
}
