use crate::expr::GenericExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult, TyMismatch};
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::LiteralKind;
use cool_parser::LiteralExpr;
use cool_resolve::expr_ty::ExprId;
use cool_resolve::ty::{FloatTy, IntTy, TyId};

#[derive(Clone, Copy, Debug)]
pub struct LiteralExprAst {
    pub id: ExprId,
    pub kind: LiteralExprKindAst,
}

impl GenericExprAst for LiteralExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl ResolveAst for LiteralExprAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        let literal_ty = self.kind.ty_id();
        let resolved_ty = literal_ty
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: literal_ty,
                expected_ty,
            })?;

        ast.expr_tys.set_expr_ty(self.id, resolved_ty);
        Ok(resolved_ty)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LiteralExprKindAst {
    Int { value: u128, ty: IntTy },
    Float { value: f64, ty: FloatTy },
    Bool(bool),
    Str(Symbol),
    CStr(Symbol),
}

impl LiteralExprKindAst {
    pub fn ty_id(&self) -> TyId {
        match self {
            Self::Int { ty, .. } => ty.ty_id(),
            Self::Float { ty, .. } => ty.ty_id(),
            _ => todo!(),
        }
    }
}

impl AstGenerator<'_> {
    pub fn gen_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralExprAst {
        let kind = match expr.literal.kind {
            LiteralKind::Number => self.gen_base_10_int(expr.literal.symbol.as_str()),
            LiteralKind::Bool => {
                if expr.literal.symbol == sym::KW_TRUE {
                    LiteralExprKindAst::Bool(true)
                } else {
                    LiteralExprKindAst::Bool(false)
                }
            }
            _ => todo!(),
        };

        LiteralExprAst {
            id: self.expr_tys.add_expr(),
            kind,
        }
    }

    pub fn gen_base_10_int(&self, number_str: &str) -> LiteralExprKindAst {
        let mut value = 0;
        let mut suffix = String::new();
        let mut char_iter = number_str.chars();

        while let Some(char) = char_iter.next() {
            if let Some(digit) = char.to_digit(10) {
                value = value * 10 + digit as u128;
            } else if char == '_' {
                continue;
            } else {
                suffix.push(char);
                break;
            }
        }

        suffix.extend(char_iter);

        LiteralExprKindAst::Int {
            value,
            ty: int_ty_from_suffix(&suffix),
        }
    }
}

fn int_ty_from_suffix(suffix: &str) -> IntTy {
    match suffix {
        "" => IntTy::Inferred,
        "i8" => IntTy::I8,
        "i16" => IntTy::I16,
        "i32" => IntTy::I32,
        "i64" => IntTy::I64,
        "i128" => IntTy::I128,
        "isize" => IntTy::Isize,
        "u8" => IntTy::U8,
        "u16" => IntTy::U16,
        "u32" => IntTy::U32,
        "u64" => IntTy::U64,
        "u128" => IntTy::U128,
        "usize" => IntTy::Usize,
        _ => todo!("handle unknown suffix: {}", suffix),
    }
}
