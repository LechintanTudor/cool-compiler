use crate::{resolve_int_literal, AstGenerator, ExprAst, SpannedAstResult, WithSpan};
use cool_collections::SmallString;
use cool_derive::Section;
use cool_lexer::sym;
use cool_parser::{LiteralExpr, LiteralKind};
use cool_resolve::{tys, ExprId, TyId};
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct LiteralExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub value: LiteralExprValue,
}

#[derive(Clone, Debug)]
pub enum LiteralExprValue {
    Int(u128),
    Float(f64),
    Bool(bool),
    Char(char),
    Cstr(SmallString),
}

impl AstGenerator<'_> {
    pub fn gen_literal_expr(
        &mut self,
        expr: &LiteralExpr,
        expected_ty_id: TyId,
    ) -> SpannedAstResult<ExprAst> {
        let (value, found_ty_id) = match expr.kind {
            LiteralKind::Int => {
                resolve_int_literal(expr.value.as_str())
                    .map(|(value, ty_id)| (LiteralExprValue::Int(value), ty_id))
                    .with_span(expr.span)?
            }
            LiteralKind::Bool => {
                (
                    LiteralExprValue::Bool(expr.value == sym::kw_true),
                    tys::bool,
                )
            }
            _ => todo!(),
        };

        self.gen_tail_expr(
            expr.span,
            found_ty_id,
            expected_ty_id,
            |context, span, ty_id| {
                LiteralExprAst {
                    span,
                    expr_id: context.add_rvalue_expr(ty_id),
                    value,
                }
            },
        )
    }
}
