use crate::{AstError, AstGenerator, AstResult, AstResultExt, ExprAst, LogicError};
use cool_parser::MatchExpr;
use cool_resolve::{BindingId, ExprId, FrameId, ResolveExpr, TyId, ValueTy};
use cool_span::{Section, Span};
use rustc_hash::FxHashSet;

#[derive(Clone, Debug)]
pub struct MatchArmAst {
    pub span: Span,
    pub arm_ty_id: TyId,
    pub binding_id: Option<BindingId>,
    pub expr: Box<ExprAst>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MatchExprKind {
    ByValue(TyId),
    ByRef(TyId),
}

impl MatchExprKind {
    #[inline]
    pub fn ty_id(&self) -> TyId {
        match self {
            Self::ByValue(ty_id) => *ty_id,
            Self::ByRef(ty_id) => *ty_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MatchExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub matched_expr: Box<ExprAst>,
    pub arms: Vec<MatchArmAst>,
    pub else_arm: Option<Box<ExprAst>>,
}

impl MatchExprAst {
    pub fn kind(&self) -> MatchExprKind {
        let matched_expr_ty_id = self.matched_expr.expr_id().ty_id;

        match matched_expr_ty_id.get_value() {
            ValueTy::Ptr(ptr_ty) => MatchExprKind::ByRef(ptr_ty.pointee),
            ValueTy::Variant(_) => MatchExprKind::ByValue(matched_expr_ty_id),
            _ => panic!(),
        }
    }

    pub fn get_variant_index(&self, variant_ty_id: TyId) -> Option<u32> {
        self.kind()
            .ty_id()
            .get_variant()
            .get_variant_index(variant_ty_id)
    }
}

impl Section for MatchExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_match_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &MatchExpr,
    ) -> AstResult<ExprAst> {
        let matched_expr = self.gen_expr(frame_id, self.tys().infer, &expr.matched_expr)?;
        let matched_expr_ty_id = matched_expr.expr_id().ty_id;

        let kind = match matched_expr_ty_id.get_value() {
            ValueTy::Ptr(ptr_ty) if ptr_ty.pointee.is_variant() => {
                MatchExprKind::ByRef(ptr_ty.pointee)
            }
            ValueTy::Variant(_) => MatchExprKind::ByValue(matched_expr_ty_id),
            _ => {
                return AstResult::ty_mismatch(
                    expr.matched_expr.span(),
                    matched_expr_ty_id,
                    self.tys().infer_variant,
                )
            }
        };

        let mut missing_variants = kind
            .ty_id()
            .get_variant()
            .variants()
            .iter()
            .copied()
            .collect::<FxHashSet<_>>();

        let mut found_ty_id = expected_ty_id;

        let (arms, else_arm) = match kind {
            MatchExprKind::ByValue(_) => {
                let mut arms = Vec::<MatchArmAst>::new();

                for arm in expr.arms.iter() {
                    let arm_ty_id = self.resolve_ty(frame_id, &arm.ty)?;

                    if !missing_variants.remove(&arm_ty_id) {
                        return AstResult::error(
                            expr.span(),
                            LogicError::InvalidVariant {
                                ty_id: matched_expr_ty_id,
                                variant_ty_id: arm_ty_id,
                            },
                        );
                    }

                    let (binding_id, expr) = match arm.pattern.as_ref() {
                        Some(pattern) => {
                            let frame_id = self.resolve.add_frame(frame_id.into());

                            let binding_id = self
                                .resolve
                                .insert_local_binding(
                                    frame_id,
                                    pattern.is_mutable,
                                    pattern.ident.symbol,
                                    Some(arm_ty_id),
                                )
                                .map_err(|error| AstError::new(expr.span(), error))?;

                            let expr =
                                self.gen_stmt_expr_or_expr(frame_id, found_ty_id, &arm.code)?;

                            (Some(binding_id), expr)
                        }
                        None => {
                            let expr =
                                self.gen_stmt_expr_or_expr(frame_id, found_ty_id, &arm.code)?;

                            (None, expr)
                        }
                    };

                    let expr_ty_id = expr.expr_id().ty_id;

                    if found_ty_id.is_infer() && expr_ty_id.is_value() {
                        found_ty_id = expr_ty_id;
                    }

                    arms.push(MatchArmAst {
                        span: arm.span(),
                        arm_ty_id,
                        binding_id,
                        expr: Box::new(expr),
                    })
                }

                let else_arm = expr
                    .else_arm
                    .as_ref()
                    .map(|else_arm| {
                        self.gen_stmt_expr_or_expr(frame_id, found_ty_id, &else_arm.code)
                    })
                    .transpose()?;

                (arms, else_arm)
            }
            MatchExprKind::ByRef(_) => todo!(),
        };

        if else_arm.is_none() && !missing_variants.is_empty() {
            return AstResult::error(
                expr.span(),
                LogicError::MissingVariants {
                    ty_id: kind.ty_id(),
                },
            );
        }

        self.resolve_expr(
            expr.span(),
            found_ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                MatchExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    matched_expr: Box::new(matched_expr),
                    arms,
                    else_arm: else_arm.map(Box::new),
                }
            },
        )
    }
}
