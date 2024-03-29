use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::{RangeExpr, RangeKind};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub enum RangeKindAst {
    Full,
    From(Box<ExprAst>),
    To(Box<ExprAst>),
    FromTo((Box<ExprAst>, Box<ExprAst>)),
}

impl RangeKindAst {
    pub fn as_from_to_pair(&self) -> (Option<&ExprAst>, Option<&ExprAst>) {
        match self {
            Self::Full => (None, None),
            Self::From(from) => (Some(from), None),
            Self::To(to) => (None, Some(to)),
            Self::FromTo((from, to)) => (Some(from), Some(to)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RangeExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub kind: RangeKindAst,
}

impl Section for RangeExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_range_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &RangeExpr,
    ) -> AstResult<ExprAst> {
        let base = self.gen_expr(frame_id, self.tys().infer, &expr.base)?;

        let kind = match &expr.kind {
            RangeKind::Full => RangeKindAst::Full,
            RangeKind::From(from) => {
                let from = self.gen_expr(frame_id, self.tys().usize, from)?;
                RangeKindAst::From(Box::new(from))
            }
            RangeKind::To(to) => {
                let to = self.gen_expr(frame_id, self.tys().usize, to)?;
                RangeKindAst::To(Box::new(to))
            }
            RangeKind::FromTo((from, to)) => {
                let from = self.gen_expr(frame_id, self.tys().usize, from)?;
                let to = self.gen_expr(frame_id, self.tys().usize, to)?;
                RangeKindAst::FromTo((Box::new(from), Box::new(to)))
            }
        };

        let base_expr_id = base.expr_id();

        let Some(value_ty) = base_expr_id.ty_id.as_value() else {
            panic!("type is not a value type");
        };

        let elem_ty_id = match value_ty {
            ValueTy::Array(array_ty) => {
                if expr.is_mutable {
                    assert!(base_expr_id.is_assignable());
                }

                array_ty.elem
            }
            ValueTy::Slice(slice_ty) => {
                if expr.is_mutable {
                    assert!(slice_ty.is_mutable);
                }

                slice_ty.elem
            }
            _ => panic!("type does not support range operations"),
        };

        let found_ty_id = self.resolve.mk_slice(elem_ty_id, expr.is_mutable);

        self.resolve_expr(
            expr.span(),
            found_ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                RangeExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    base: Box::new(base),
                    kind,
                }
            },
        )
    }
}
