use crate::{AstGenerator, AstResult, ExprAst};
use cool_lexer::symbols::Symbol;
use cool_parser::{Ident, StructExpr};
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};
use rustc_hash::FxHashSet;

#[derive(Clone, Debug)]
pub struct StructFieldInitializerAst {
    pub ident: Ident,
    pub expr: Box<ExprAst>,
}

#[derive(Clone, Debug)]
pub struct StructExprAst {
    pub expr_id: ExprId,
    pub initializers: Vec<StructFieldInitializerAst>,
}

impl AstGenerator<'_> {
    pub fn gen_struct_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &StructExpr,
    ) -> AstResult<StructExprAst> {
        let ty_id = self
            .gen_expr(frame_id, tys::TY, &expr.base)?
            .as_ty()
            .expect("struct base is not a type")
            .item_ty_id;

        let struct_ty = self.resolve[ty_id]
            .ty
            .as_struct()
            .expect("struct base is not a struct type")
            .clone();

        let mut initializers = Vec::<StructFieldInitializerAst>::new();
        let mut used_fields = FxHashSet::<Symbol>::default();

        for initializer in expr.initializers.iter() {
            let field_ty_id = struct_ty
                .fields
                .iter()
                .find(|(symbol, _)| *symbol == initializer.ident.symbol)
                .map(|(_, field_ty_id)| *field_ty_id)
                .expect("unknown struct field in initializer");

            let is_duplicate = !used_fields.insert(initializer.ident.symbol);

            if is_duplicate {
                panic!(
                    "duplicate field initializer: {}",
                    initializer.ident.symbol.as_str()
                );
            }

            let expr = self.gen_expr(frame_id, field_ty_id, &initializer.expr)?;

            initializers.push(StructFieldInitializerAst {
                ident: initializer.ident,
                expr: Box::new(expr),
            });
        }

        if initializers.len() < struct_ty.fields.len() {
            panic!("missing struct fields");
        }

        let ty_id = self.resolve.resolve_direct_ty_id(ty_id, expected_ty_id)?;

        Ok(StructExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            initializers,
        })
    }
}
