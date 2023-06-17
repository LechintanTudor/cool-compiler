use crate::{AstError, AstGenerator, AstResult, AstResultExt, LiteralError, LiteralErrorKind};
use cool_collections::SmallString;
use cool_lexer::{sym, LiteralKind, Symbol};
use cool_parser::LiteralExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyConsts, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub enum LiteralExprValue {
    Int(u128),
    Float(f64),
    Bool(bool),
    Char(u32),
    Cstr(SmallString),
}

#[derive(Clone, Debug)]
pub struct LiteralExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub value: LiteralExprValue,
}

impl LiteralExprAst {
    #[inline]
    pub fn as_int_value(&self) -> Option<u128> {
        match self.value {
            LiteralExprValue::Int(value) => Some(value),
            _ => None,
        }
    }
}

impl Section for LiteralExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_literal_expr(
        &mut self,
        _frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &LiteralExpr,
    ) -> AstResult<LiteralExprAst> {
        let tys = self.tys();

        let expr = match expr.literal.kind {
            LiteralKind::Int { .. } => {
                let (value, ty_id) = parse_int(tys, expr)?;
                let ty_id = self.resolve.resolve_direct_ty_id(ty_id, expected_ty_id)?;

                if !is_int_in_range(tys, value, ty_id) {
                    return AstResult::error(
                        expr.span(),
                        LiteralError {
                            literal: expr.literal.symbol,
                            kind: LiteralErrorKind::IntOutOfRange { ty_id },
                        },
                    );
                }

                if ty_id.is_int() {
                    LiteralExprAst {
                        span: expr.span,
                        expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                        value: LiteralExprValue::Int(value),
                    }
                } else {
                    LiteralExprAst {
                        span: expr.span,
                        expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                        value: LiteralExprValue::Float(value as _),
                    }
                }
            }
            LiteralKind::Decimal => {
                let (value, ty_id) = parse_decimal(tys, expr)?;
                let ty_id = self.resolve.resolve_direct_ty_id(ty_id, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Float(value),
                }
            }
            LiteralKind::Bool => {
                let value = expr.literal.symbol == sym::KW_TRUE;
                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(self.tys().bool, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Bool(value),
                }
            }
            LiteralKind::Char => {
                let value = parse_char(expr.literal.symbol);
                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(self.tys().char, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Char(value),
                }
            }
            LiteralKind::Str => {
                let value = parse_str(expr.literal.symbol);
                let ty_id = self
                    .resolve
                    .resolve_direct_ty_id(tys.c_str, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Cstr(value),
                }
            }
        };

        Ok(expr)
    }
}

fn parse_int(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(u128, TyId)> {
    let literal_str = expr.literal.symbol.as_str();

    if literal_str.starts_with("0b") {
        parse_binary_int(tys, expr)
    } else if literal_str.starts_with("0o") {
        parse_octal_int(tys, expr)
    } else if literal_str.starts_with("0x") {
        parse_hexadecimal_int(tys, expr)
    } else {
        parse_decimal_int(tys, expr)
    }
}

fn parse_binary_int(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(u128, TyId)> {
    let literal_str = expr.literal.symbol.as_str();
    let mut char_iter = literal_str.chars();

    // Skip "0b" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = SmallString::new();

    for char in char_iter.by_ref() {
        match char {
            '0' | '1' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 2, digit).ok_or_else(|| {
                    AstError::new(
                        expr.span(),
                        LiteralError {
                            literal: expr.literal.symbol,
                            kind: LiteralErrorKind::IntOutOfRange { ty_id: tys.i128 },
                        },
                    )
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                suffix.extend(char_iter);
                break;
            }
        }
    }

    let Some(suffix) = parse_int_suffix(tys, &suffix) else {
        return AstResult::error(expr.span(), LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::UnknownSuffix {
                    suffix: Symbol::insert(&suffix),
                },
            }) ;
    };

    Ok((value, suffix))
}

fn parse_octal_int(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(u128, TyId)> {
    let literal_str = expr.literal.symbol.as_str();
    let mut char_iter = literal_str.chars();

    // Skip "0o" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='7' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 8, digit).ok_or_else(|| {
                    AstError::new(
                        expr.span(),
                        LiteralError {
                            literal: expr.literal.symbol,
                            kind: LiteralErrorKind::IntOutOfRange { ty_id: tys.i128 },
                        },
                    )
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                suffix.extend(char_iter);
                break;
            }
        }
    }

    let Some(suffix) = parse_int_suffix(tys, &suffix) else {
        return AstResult::error(expr.span(), LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::UnknownSuffix {
                    suffix: Symbol::insert(&suffix),
                },
            }) ;
    };

    Ok((value, suffix))
}

fn parse_hexadecimal_int(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(u128, TyId)> {
    let literal_str = expr.literal.symbol.as_str();
    let mut char_iter = literal_str.chars();

    // Skip "0x" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    let int_out_of_range = || {
        AstError::new(
            expr.span(),
            LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::IntOutOfRange { ty_id: tys.i128 },
            },
        )
    };

    for char in char_iter.by_ref() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 16, digit).ok_or_else(int_out_of_range)?;
            }
            'a'..='f' => {
                let digit = char as u32 - 'a' as u32 + 10;
                value = append_digit(value, 16, digit).ok_or_else(int_out_of_range)?;
            }
            'A'..='F' => {
                let digit = char as u32 - 'A' as u32 + 10;
                value = append_digit(value, 16, digit).ok_or_else(int_out_of_range)?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                suffix.extend(char_iter);
                break;
            }
        }
    }

    let Some(suffix) = parse_int_suffix(tys, &suffix) else {
        return AstResult::error(expr.span(), LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::UnknownSuffix {
                    suffix: Symbol::insert(&suffix),
                },
            }) ;
    };

    Ok((value, suffix))
}

fn parse_decimal_int(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(u128, TyId)> {
    let literal_str = expr.literal.symbol.as_str();
    let mut char_iter = literal_str.chars();

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 10, digit).ok_or_else(|| {
                    AstError::new(
                        expr.span(),
                        LiteralError {
                            literal: expr.literal.symbol,
                            kind: LiteralErrorKind::IntOutOfRange { ty_id: tys.i128 },
                        },
                    )
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    let Some(suffix) = parse_int_suffix(tys, &suffix) else {
        return AstResult::error(expr.span(), LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::UnknownSuffix {
                    suffix: Symbol::insert(&suffix),
                },
        }) ;
    };

    Ok((value, suffix))
}

fn append_digit(value: u128, base: u128, digit: u32) -> Option<u128> {
    value
        .checked_mul(base)
        .and_then(|value| value.checked_add(digit as u128))
}

fn parse_int_suffix(tys: &TyConsts, suffix: &str) -> Option<TyId> {
    let ty_id = match suffix {
        "" => tys.infer_int,
        "i8" => tys.i8,
        "i16" => tys.i16,
        "i32" => tys.i32,
        "i64" => tys.i64,
        "i128" => tys.i128,
        "isize" | "i" => tys.isize,
        "u8" => tys.u8,
        "u16" => tys.u16,
        "u32" => tys.u32,
        "u64" => tys.u64,
        "u128" => tys.u128,
        "usize" | "u" => tys.usize,
        _ => return None,
    };

    Some(ty_id)
}

pub fn parse_decimal_suffix(tys: &TyConsts, suffix: &str) -> Option<TyId> {
    let ty_id = match suffix {
        "" => tys.infer_float,
        "f32" => tys.f32,
        "f64" => tys.f64,
        _ => return None,
    };

    Some(ty_id)
}

pub fn parse_number_suffix(tys: &TyConsts, suffix: &str) -> Option<TyId> {
    let ty_id = match suffix {
        "" => tys.infer_int,
        "i8" => tys.i8,
        "i16" => tys.i16,
        "i32" => tys.i32,
        "i64" => tys.i64,
        "i128" => tys.i128,
        "isize" | "i" => tys.isize,
        "u8" => tys.u8,
        "u16" => tys.u16,
        "u32" => tys.u32,
        "u64" => tys.u64,
        "u128" => tys.u128,
        "usize" | "u" => tys.usize,
        "f32" => tys.f32,
        "f64" => tys.f64,
        _ => return None,
    };

    Some(ty_id)
}

fn is_int_in_range(tys: &TyConsts, value: u128, ty_id: TyId) -> bool {
    if ty_id == tys.infer_int {
        return true;
    }

    let rhs_value: u128 = if ty_id == tys.i8 {
        i8::MAX as _
    } else if ty_id == tys.i16 {
        i16::MAX as _
    } else if ty_id == tys.i32 {
        i32::MAX as _
    } else if ty_id == tys.i64 {
        i64::MAX as _
    } else if ty_id == tys.i128 {
        i128::MAX as _
    } else if ty_id == tys.u8 {
        u8::MAX as _
    } else if ty_id == tys.u16 {
        u16::MAX as _
    } else if ty_id == tys.u32 {
        u32::MAX as _
    } else if ty_id == tys.u64 {
        u64::MAX as _
    } else if ty_id == tys.u128 {
        u128::MAX as _
    } else if ty_id == tys.f32 {
        f32::MAX as _
    } else if ty_id == tys.f64 {
        f64::MAX as _
    } else {
        return true;
    };

    value <= rhs_value
}

fn parse_decimal(tys: &TyConsts, expr: &LiteralExpr) -> AstResult<(f64, TyId)> {
    let value_str = expr.literal.symbol.as_str();
    let mut char_iter = value_str.chars();

    let mut value = 0.0;
    let mut divider = 10.0;
    let mut found_dot = false;
    let mut suffix = SmallString::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;

                if !found_dot {
                    value = value * 10.0 + digit as f64;
                } else {
                    let digit_value = (digit as f64) / divider;
                    divider *= 10.0;
                    value += digit_value;
                }
            }
            '_' => (),
            '.' => found_dot = true,
            _ => {
                suffix.extend(char_iter);
                suffix.push(char);
                break;
            }
        }
    }

    let Some(suffix) = parse_decimal_suffix(tys, &suffix) else {
        return AstResult::error(expr.span(), LiteralError {
                literal: expr.literal.symbol,
                kind: LiteralErrorKind::UnknownSuffix {
                    suffix: Symbol::insert(&suffix),
                },
        }) ;
    };

    Ok((value, suffix))
}

fn parse_char(symbol: Symbol) -> u32 {
    let char_str = symbol.as_str();
    let mut char_iter = char_str.chars();

    let char = match char_iter.next().unwrap() {
        '\\' => {
            match char_iter.next().unwrap() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '0' => '\0',
                '\'' => '\'',
                _ => unreachable!(),
            }
        }
        char => char,
    };

    char as u32
}

fn parse_str(symbol: Symbol) -> SmallString {
    let str = symbol.as_str();
    let mut char_iter = str.chars();
    let mut result = SmallString::new();

    while let Some(char) = char_iter.next() {
        let char = match char {
            '\\' => {
                match char_iter.next().unwrap() {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '0' => '\0',
                    '"' => '"',
                    _ => unreachable!(),
                }
            }
            _ => char,
        };

        result.push(char);
    }

    result
}
