use crate::expr::GenericExprAst;
use crate::{AstGenerator, AstResult, ResolveAst, TyMismatch};
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::LiteralKind;
use cool_parser::LiteralExpr;
use cool_resolve::{tys, ExprId, TyId};

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
    fn resolve_consts(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.resolve_exprs(ast, expected_ty)
    }

    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let literal_ty = self.kind.ty_id();
        let resolved_ty = literal_ty
            .resolve_non_inferred(expected_ty)
            .ok_or(TyMismatch {
                found_ty: literal_ty,
                expected_ty,
            })?;

        ast.resolve.set_expr_ty(self.id, resolved_ty);
        Ok(resolved_ty)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LiteralExprKindAst {
    Int { value: u128, ty_id: TyId },
    Float { value: f64, ty_id: TyId },
    Bool(bool),
    Char(char),
    Str(Symbol),
    CStr(Symbol),
}

impl LiteralExprKindAst {
    pub fn ty_id(&self) -> TyId {
        match self {
            Self::Int { ty_id, .. } => *ty_id,
            Self::Float { ty_id, .. } => *ty_id,
            Self::Bool(_) => tys::BOOL,
            Self::Char(_) => tys::CHAR,
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
            id: self.resolve.add_expr(),
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
            ty_id: int_ty_id_from_suffix(&suffix),
        }
    }
}

fn int_ty_id_from_suffix(suffix: &str) -> TyId {
    match suffix {
        "" => tys::INFERRED_INT,
        "i8" => tys::I8,
        "i16" => tys::I16,
        "i32" => tys::I32,
        "i64" => tys::I64,
        "i128" => tys::I128,
        "isize" => tys::ISIZE,
        "u8" => tys::U8,
        "u16" => tys::U16,
        "u32" => tys::U32,
        "u64" => tys::U64,
        "u128" => tys::U128,
        "usize" => tys::USIZE,
        _ => todo!("handle unknown suffix: {}", suffix),
    }
}
