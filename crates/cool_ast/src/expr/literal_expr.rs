use crate::{AstError, AstGenerator, AstResult, AstResultExt, LiteralError, LiteralErrorKind};
use cool_collections::SmallString;
use cool_lexer::{sym, IntBase, LiteralKind, Symbol};
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
                let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;

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
                let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Float(value),
                }
            }
            LiteralKind::Bool => {
                let value = expr.literal.symbol == sym::KW_TRUE;
                let ty_id =
                    self.resolve_direct_ty_id(expr.span(), self.tys().bool, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Bool(value),
                }
            }
            LiteralKind::Char => {
                let value = parse_char(expr.literal.symbol);
                let ty_id =
                    self.resolve_direct_ty_id(expr.span(), self.tys().char, expected_ty_id)?;

                LiteralExprAst {
                    span: expr.span,
                    expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value: LiteralExprValue::Char(value),
                }
            }
            LiteralKind::Str => {
                let value = parse_str(expr.literal.symbol);
                let ty_id = self.resolve_direct_ty_id(expr.span(), tys.c_str, expected_ty_id)?;

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
    let (skip_chars, base, kind) = match expr.literal.kind {
        LiteralKind::Int { base, .. } => {
            match base {
                IntBase::B2 => (2, 2, NumberKind::Int),
                IntBase::B8 => (2, 8, NumberKind::Int),
                IntBase::B10 => (0, 10, NumberKind::Any),
                IntBase::B16 => (2, 16, NumberKind::Int),
            }
        }
        _ => (2, 10, NumberKind::Any),
    };

    let mut char_iter = expr.literal.symbol.as_str().chars();
    for _ in 0..skip_chars {
        char_iter.next();
    }

    let mut value = 0;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        let digit = match char {
            '0'..='1' if base >= 2 => char as u32 - '0' as u32,
            '2'..='7' if base >= 8 => char as u32 - '0' as u32,
            '8'..='9' if base >= 10 => char as u32 - '0' as u32,
            'a'..='f' if base >= 16 => char as u32 - 'a' as u32,
            'A'..='F' if base >= 16 => char as u32 - 'A' as u32,
            '_' => continue,
            _ => {
                suffix.push(char);
                suffix.extend(char_iter);
                break;
            }
        };

        value = append_digit(value, base, digit).ok_or_else(|| {
            AstError::new(
                expr.span(),
                LiteralError {
                    literal: expr.literal.symbol,
                    kind: LiteralErrorKind::IntOutOfRange { ty_id: tys.i128 },
                },
            )
        })?
    }

    let Some(ty_id) = parse_suffix(tys, &suffix, kind) else {
        return AstResult::error(expr.span(), LiteralError {
            literal: expr.literal.symbol,
            kind: LiteralErrorKind::UnknownSuffix {
                suffix: Symbol::insert(&suffix),
            },
        });
    };

    Ok((value, ty_id))
}

fn append_digit(value: u128, base: u128, digit: u32) -> Option<u128> {
    value
        .checked_mul(base)
        .and_then(|value| value.checked_add(digit as u128))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum NumberKind {
    Any,
    Int,
    Decimal,
}

fn parse_suffix(tys: &TyConsts, suffix: &str, kind: NumberKind) -> Option<TyId> {
    if suffix.is_empty() {
        let ty_id = match kind {
            NumberKind::Any | NumberKind::Int => tys.infer_int,
            NumberKind::Decimal => tys.infer_float,
        };

        return Some(ty_id);
    }

    let ty_id = match suffix {
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

    if kind == NumberKind::Int && !ty_id.is_int() {
        return None;
    }

    if kind == NumberKind::Decimal && !ty_id.is_float() {
        return None;
    }

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
    let mut char_iter = expr.literal.symbol.as_str().chars();

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

    let Some(ty_id) = parse_suffix(tys, &suffix, NumberKind::Decimal) else {
        return AstResult::error(expr.span(), LiteralError {
            literal: expr.literal.symbol,
            kind: LiteralErrorKind::UnknownSuffix {
                suffix: Symbol::insert(&suffix),
            },
        });
    };

    Ok((value, ty_id))
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
