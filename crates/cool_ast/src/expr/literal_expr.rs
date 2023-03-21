use crate::AstGenerator;
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::{LiteralKind, Radix};
use cool_parser::LiteralExpr;
use cool_resolve::ty::{FloatTy, IntTy};

#[derive(Clone, Debug)]
pub enum LiteralExprAst {
    Integer { value: u128, ty: Option<IntTy> },
    Float { value: f64, ty: Option<FloatTy> },
    Bool(bool),
    Str(Symbol),
    CStr(Symbol),
}

impl AstGenerator<'_> {
    pub fn generate_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralExprAst {
        match expr.literal.kind {
            LiteralKind::Integer { radix: Radix::Ten } => {
                self.generate_base_10_integer(expr.literal.symbol.as_str())
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

    pub fn generate_base_10_integer(&self, number_str: &str) -> LiteralExprAst {
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

        LiteralExprAst::Integer {
            value,
            ty: int_ty_from_suffix(&suffix),
        }
    }
}

pub fn int_ty_from_suffix(suffix: &str) -> Option<IntTy> {
    let ty = match suffix {
        "" => return None,
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
    };

    Some(ty)
}
