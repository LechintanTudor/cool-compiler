use crate::{AstGenerator, AstResult, ItemAst, ResolveAst};
use cool_lexer::symbols::Symbol;
use cool_resolve::{tys, TyId};

#[derive(Clone, Debug)]
pub struct ItemDeclAst {
    pub symbol: Symbol,
    pub explicit_ty_id: TyId,
    pub item: ItemAst,
}

impl ResolveAst for ItemDeclAst {
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        debug_assert!(expected_ty == tys::INFERRED);
        self.item.resolve_exprs(ast, self.explicit_ty_id)
    }
}
