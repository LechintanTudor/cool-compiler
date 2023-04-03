use crate::expr::GenericExprAst;
use crate::{AstGenerator, Unify};
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::{LiteralKind, Radix};
use cool_parser::LiteralExpr;
use cool_resolve::expr_ty::{Constraint, ExprId, ExprTyUnifier};
use cool_resolve::ty::{FloatTy, IntTy, TyTable};

#[derive(Clone, Copy, Debug)]
pub struct LiteralExprAst {
    pub id: ExprId,
    pub kind: LiteralExprKindAst,
}

impl Unify for LiteralExprAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, _tys: &mut TyTable) {
        let ty_id = match self.kind {
            LiteralExprKindAst::Int { ty, .. } => ty.ty_id(),
            LiteralExprKindAst::Float { ty, .. } => ty.ty_id(),
            _ => todo!(),
        };

        unifier.add_constraint(Constraint::Expr(self.id), Constraint::Ty(ty_id));
    }
}

impl GenericExprAst for LiteralExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
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

impl AstGenerator<'_> {
    pub fn gen_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralExprAst {
        let kind = match expr.literal.kind {
            LiteralKind::Int { radix: Radix::Ten } => {
                self.gen_base_10_int(expr.literal.symbol.as_str())
            }
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
            id: self.unification.add_expr(),
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
