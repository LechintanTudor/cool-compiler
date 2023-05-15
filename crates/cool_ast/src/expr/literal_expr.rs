use crate::{AstGenerator, AstResult, LiteralIntOutOfRange, LiteralUnknownSuffix, TyMismatch};
use cool_collections::SmallString;
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::LiteralKind;
use cool_parser::LiteralExpr;
use cool_resolve::{tys, ExprId, ResolveExpr, TyId};

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

impl AstGenerator<'_> {
    pub fn gen_literal_expr(
        &mut self,
        expected_ty_id: TyId,
        literal_expr: &LiteralExpr,
    ) -> AstResult<LiteralExprAst> {
        let symbol = literal_expr.literal.symbol;
        let expr = match literal_expr.literal.kind {
            LiteralKind::Int { .. } => {
                let (value, ty_id) = parse_int(symbol)?;

                let ty_id = ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: ty_id,
                        expected: expected_ty_id,
                    })?;

                if !is_int_in_range(value, ty_id) {
                    Err(LiteralIntOutOfRange { ty_id, symbol })?;
                }

                if ty_id.is_int() {
                    LiteralExprAst {
                        expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                        value: LiteralExprValue::Int(value),
                    }
                } else {
                    LiteralExprAst {
                        expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                        value: LiteralExprValue::Float(value as _),
                    }
                }
            }
            LiteralKind::Decimal => {
                let (value, ty_id) = parse_decimal(symbol)?;

                let ty_id = ty_id
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: ty_id,
                        expected: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Float(value),
                }
            }
            LiteralKind::Bool => {
                let value = symbol == sym::KW_TRUE;

                let ty_id = tys::BOOL
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::BOOL,
                        expected: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Bool(value),
                }
            }
            LiteralKind::Char => {
                let value = parse_char(symbol);

                let ty_id = tys::CHAR
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::CHAR,
                        expected: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Char(value),
                }
            }
            LiteralKind::Str => {
                let value = parse_str(symbol);

                let ty_id = tys::C_STR
                    .resolve_non_inferred(expected_ty_id)
                    .ok_or(TyMismatch {
                        found: tys::C_STR,
                        expected: expected_ty_id,
                    })?;

                LiteralExprAst {
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Cstr(value),
                }
            }
        };

        Ok(expr)
    }
}

fn parse_int(symbol: Symbol) -> AstResult<(u128, TyId)> {
    let value_str = symbol.as_str();

    if value_str.starts_with("0b") {
        parse_binary_int(symbol)
    } else if value_str.starts_with("0o") {
        parse_octal_int(symbol)
    } else if value_str.starts_with("0x") {
        parse_hexadecimal_int(symbol)
    } else {
        parse_decimal_int(symbol)
    }
}

fn parse_binary_int(symbol: Symbol) -> AstResult<(u128, TyId)> {
    let value_str = symbol.as_str();
    let mut char_iter = value_str.chars();

    // Skip "0b" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = SmallString::new();

    for char in char_iter.by_ref() {
        match char {
            '0' | '1' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 2, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);
    Ok((value, parse_int_suffix(&suffix)?))
}

fn parse_octal_int(symbol: Symbol) -> AstResult<(u128, TyId)> {
    let value_str = symbol.as_str();
    let mut char_iter = value_str.chars();

    // Skip "0o" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='7' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 8, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);
    Ok((value, parse_int_suffix(&suffix)?))
}

fn parse_hexadecimal_int(symbol: Symbol) -> AstResult<(u128, TyId)> {
    let value_str = symbol.as_str();
    let mut char_iter = value_str.chars();

    // Skip "0x" prefix
    char_iter.next();
    char_iter.next();

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 16, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            'a'..='f' => {
                let digit = char as u32 - 'a' as u32 + 10;
                value = append_digit(value, 16, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            'A'..='F' => {
                let digit = char as u32 - 'A' as u32 + 10;
                value = append_digit(value, 16, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);
    Ok((value, parse_int_suffix(&suffix)?))
}

fn parse_decimal_int(symbol: Symbol) -> AstResult<(u128, TyId)> {
    let value_str = symbol.as_str();
    let mut char_iter = value_str.chars();

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        match char {
            '0'..='9' => {
                let digit = char as u32 - '0' as u32;
                value = append_digit(value, 10, digit).ok_or(LiteralIntOutOfRange {
                    ty_id: tys::U128,
                    symbol,
                })?;
            }
            '_' => (),
            _ => {
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);
    Ok((value, parse_number_suffix(&suffix)?))
}

fn append_digit(value: u128, base: u128, digit: u32) -> Option<u128> {
    value
        .checked_mul(base)
        .and_then(|value| value.checked_add(digit as u128))
}

fn parse_int_suffix(suffix: &str) -> AstResult<TyId> {
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
        _ => {
            Err(LiteralUnknownSuffix {
                suffix: Symbol::insert(suffix),
            })?
        }
    };

    Ok(ty_id)
}

pub fn parse_decimal_suffix(suffix: &str) -> AstResult<TyId> {
    let ty_id = match suffix {
        "" => tys::INFERRED_FLOAT,
        "f32" => tys::F32,
        "f64" => tys::F64,
        _ => {
            Err(LiteralUnknownSuffix {
                suffix: Symbol::insert(suffix),
            })?
        }
    };

    Ok(ty_id)
}

pub fn parse_number_suffix(suffix: &str) -> AstResult<TyId> {
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
        "f32" => tys::F32,
        "f64" => tys::F64,
        _ => {
            Err(LiteralUnknownSuffix {
                suffix: Symbol::insert(suffix),
            })?
        }
    };

    Ok(ty_id)
}

fn is_int_in_range(value: u128, ty_id: TyId) -> bool {
    match ty_id {
        tys::INFERRED_INT => true,
        tys::I8 => value <= i8::MAX as _,
        tys::I16 => value <= i16::MAX as _,
        tys::I32 => value <= i32::MAX as _,
        tys::I64 => value <= i64::MAX as _,
        tys::I128 => value <= i128::MAX as _,
        tys::U8 => value <= u8::MAX as _,
        tys::U16 => value <= u16::MAX as _,
        tys::U32 => value <= u32::MAX as _,
        tys::U64 => value <= u64::MAX as _,
        tys::U128 => value <= u128::MAX as _,
        tys::F32 => value <= f32::MAX as _,
        tys::F64 => value <= f64::MAX as _,
        _ => todo!(),
    }
}

pub fn parse_decimal(symbol: Symbol) -> AstResult<(f64, TyId)> {
    let value_str = symbol.as_str();
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
                suffix.push(char);
                break;
            }
        }
    }

    suffix.extend(char_iter);
    Ok((value, parse_decimal_suffix(&suffix)?))
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
