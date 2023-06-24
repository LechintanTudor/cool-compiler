use crate::{AstGenerator, AstResult, ExprAst};
use cool_lexer::Symbol;
use cool_parser::{Ident, StructExpr};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};
use rustc_hash::FxHashSet;

#[derive(Clone, Debug)]
pub struct StructFieldInitializerAst {
    pub span: Span,
    pub ident: Ident,
    pub expr: Box<ExprAst>,
}

impl Section for StructFieldInitializerAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct StructExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub initializers: Vec<StructFieldInitializerAst>,
}

impl Section for StructExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_struct_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &StructExpr,
    ) -> AstResult<StructExprAst> {
        let ty_id = self
            .gen_expr(frame_id, self.tys().ty, &expr.base)?
            .as_ty()
            .expect("struct base is not a type")
            .item_ty_id;

        ty_id
            .shape
            .as_struct()
            .expect("struct base is not a struct type");

        let mut initializers = Vec::<StructFieldInitializerAst>::new();
        let mut used_fields = FxHashSet::<Symbol>::default();
        let fields = ty_id.def.get_aggregate_fields().unwrap();

        for initializer in expr.initializers.iter() {
            let field_ty_id = fields
                .iter()
                .find(|field| field.symbol == initializer.ident.symbol)
                .map(|field| field.ty_id)
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
                span: initializer.span(),
                ident: initializer.ident,
                expr: Box::new(expr),
            });
        }

        if initializers.len() < fields.len() {
            panic!("missing struct fields");
        }

        let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;

        Ok(StructExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            initializers,
        })
    }
}
