use crate::AstGenerator;
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::{LiteralKind, Radix};
use cool_parser::LiteralExpr;
use cool_resolve::ty::{FloatTy, IntTy, TyId};

#[derive(Clone, Debug)]
pub enum LiteralExprAst {
    Int { value: u128, ty: IntTy },
    Float { value: f64, ty: FloatTy },
    Bool(bool),
    Str(Symbol),
    CStr(Symbol),
}

impl LiteralExprAst {
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
        match expr.literal.kind {
            LiteralKind::Int { radix: Radix::Ten } => {
                self.gen_base_10_int(expr.literal.symbol.as_str())
            }
            LiteralKind::Bool => {
                if expr.literal.symbol == sym::KW_TRUE {
                    LiteralExprAst::Bool(true)
                } else {
                    LiteralExprAst::Bool(false)
                }
            }
            _ => todo!(),
        }
    }

    pub fn gen_base_10_int(&self, number_str: &str) -> LiteralExprAst {
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
            }
        }

        suffix.extend(char_iter);

        LiteralExprAst::Int {
            value,
            ty: int_ty_from_suffix(&suffix),
        }
    }
}

pub fn int_ty_from_suffix(suffix: &str) -> IntTy {
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
        _ => todo!("handle unknown suffix"),
    }
}
