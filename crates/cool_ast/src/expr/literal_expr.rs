use crate::{AstGenerator, AstResult, TyMismatch};
use cool_lexer::symbols::sym;
use cool_lexer::tokens::LiteralKind;
use cool_parser::LiteralExpr;
use cool_resolve::{tys, ExprId, TyId};

#[derive(Clone, Debug)]
pub enum LiteralExprValue {
    Int(u128),
    Float(f64),
    Bool(bool),
    Char(u32),
    Cstr(String),
}

#[derive(Clone, Debug)]
pub struct LiteralExprAst {
    pub expr_id: ExprId,
    pub value: LiteralExprValue,
}

impl AstGenerator<'_> {
    pub fn gen_literal_expr(
        &mut self,
        expected_ty_id: TyId,
        literal_expr: &LiteralExpr,
    ) -> AstResult<LiteralExprAst> {
        let expr = match literal_expr.literal.kind {
            LiteralKind::Number => {
                let value_str = literal_expr.literal.symbol.as_str();
                let (value, ty_id) = parse_int(value_str).unwrap();

                let ty_id = ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found_ty: ty_id,
                        expected_ty: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ty_id),
                    value: LiteralExprValue::Int(value),
                }
            }
            LiteralKind::Bool => {
                let value = literal_expr.literal.symbol == sym::KW_TRUE;

                let ty_id = tys::BOOL
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found_ty: tys::BOOL,
                        expected_ty: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ty_id),
                    value: LiteralExprValue::Bool(value),
                }
            }
            LiteralKind::Char => {
                let value_str = literal_expr.literal.symbol.as_str();
                let value = parse_char(value_str).unwrap();

                let ty_id = tys::CHAR
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found_ty: tys::CHAR,
                        expected_ty: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ty_id),
                    value: LiteralExprValue::Char(value),
                }
            }
            LiteralKind::Str => {
                let value_str = literal_expr.literal.symbol.as_str();
                let value = value_str.to_owned();

                let ty_id = tys::C_STR
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found_ty: tys::C_STR,
                        expected_ty: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ty_id),
                    value: LiteralExprValue::Cstr(value),
                }
            }
        };

        Ok(expr)
    }
}

fn parse_int(value_str: &str) -> Option<(u128, TyId)> {
    if value_str.starts_with("0b") {
        parse_binary_int(value_str)
    } else if value_str.starts_with("0o") {
        parse_octal_int(value_str)
    } else if value_str.starts_with("0x") {
        parse_hexadecimal_int(value_str)
    } else {
        parse_decimal_int(value_str)
    }
}

fn parse_binary_int(value_str: &str) -> Option<(u128, TyId)> {
    let mut char_iter = value_str.chars();
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    while let Some(char) = char_iter.next() {
        match char {
            '0' | '1' => {
                let digit = char as u32 - '0' as u32;
                value = (value * 2) + digit as u128;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);

    Some((value, parse_int_suffix(&suffix)?))
}

fn parse_octal_int(value_str: &str) -> Option<(u128, TyId)> {
    let mut char_iter = value_str.chars();
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    while let Some(char) = char_iter.next() {
        match char {
            '0'..='7' => {
                let digit = char as u32 - '0' as u32;
                value = (value * 8) + digit as u128;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);

    Some((value, parse_int_suffix(&suffix)?))
}

fn parse_hexadecimal_int(value_str: &str) -> Option<(u128, TyId)> {
    let mut char_iter = value_str.chars();
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    while let Some(char) = char_iter.next() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = (value * 16) + digit as u128;
            }
            'a'..='f' => {
                let digit = char as u32 - 'a' as u32;
                value = (value * 16) + digit as u128;
            }
            'A'..='F' => {
                let digit = char as u32 - 'A' as u32;
                value = (value * 16) + digit as u128;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);

    Some((value, parse_int_suffix(&suffix)?))
}

fn parse_decimal_int(value_str: &str) -> Option<(u128, TyId)> {
    let mut char_iter = value_str.chars();
    let mut value = 0;
    let mut suffix = String::new();

    while let Some(char) = char_iter.next() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = (value * 10) + digit as u128;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);

    Some((value, parse_int_suffix(&suffix)?))
}

fn parse_int_suffix(suffix: &str) -> Option<TyId> {
    let ty_id = match suffix {
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
        _ => return None,
    };

    Some(ty_id)
}

fn parse_char(char_str: &str) -> Option<u32> {
    char_str.chars().next().map(|c| c as u32)
}
