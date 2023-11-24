use crate::{AstGenerator, ExprAst, SpannedAstResult, WithSpan};
use cool_derive::Section;
use cool_lexer::Symbol;
use cool_parser::Ident;
use cool_resolve::{tys, Expr, ExprId, ExprKind, FrameId, ItemKind, TyId};
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct IdentExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub symbol: Symbol,
}

impl AstGenerator<'_> {
    pub fn gen_ident_expr(
        &mut self,
        ident: &Ident,
        frame_id: FrameId,
        expected_ty_id: TyId,
    ) -> SpannedAstResult<ExprAst> {
        let item_kind = self
            .context
            .get_symbol(frame_id, ident.symbol)
            .with_span(ident.span)?;

        let (ty_id, kind) = match item_kind {
            ItemKind::Binding(binding_id) => {
                let binding = &self.context[binding_id];

                (
                    binding.ty_id,
                    ExprKind::Lvalue {
                        is_mutable: binding.is_mutable,
                    },
                )
            }
            ItemKind::Const(const_id) => (self.context[const_id].ty_id, ExprKind::Rvalue),
            ItemKind::Module(_) => (tys::module, ExprKind::Lvalue { is_mutable: false }),
            ItemKind::Ty(_) => (tys::alias, ExprKind::Lvalue { is_mutable: false }),
        };

        self.gen_tail_expr(ident.span, ty_id, expected_ty_id, |context, span, ty_id| {
            IdentExprAst {
                span,
                expr_id: context.add_expr(Expr { ty_id, kind }),
                symbol: ident.symbol,
            }
        })
    }
}
