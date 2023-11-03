use crate::{LiteralError, LiteralResult};
use cool_resolve::{tys, TyId};

pub fn resolve_int_literal(literal_str: &str) -> LiteralResult<(u128, TyId)> {
    let base = if literal_str.starts_with("0b") {
        Base::Binary
    } else if literal_str.starts_with("0o") {
        Base::Octal
    } else if literal_str.starts_with("0x") {
        Base::Hexadecimal
    } else {
        Base::Decimal
    };

    let mut char_iter = literal_str.chars();
    let base_value = base.value() as u128;

    if base != Base::Decimal {
        for _ in 0..2 {
            let _ = char_iter.next();
        }
    }

    let mut value = 0_u128;
    let mut suffix = String::new();

    for char in char_iter.by_ref() {
        if char == '_' {
            continue;
        }

        let Some(digit_value) = base.digit_value(char) else {
            suffix.push(char);
            suffix.extend(char_iter);
            break;
        };

        value = value
            .checked_mul(base_value)
            .ok_or(LiteralError::ValueTooLarge)?
            .checked_add(digit_value as _)
            .ok_or(LiteralError::ValueTooLarge)?;
    }

    Ok((value, resolve_suffix(&suffix)?))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Base {
    fn value(&self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hexadecimal => 16,
        }
    }

    fn digit_value(&self, digit: char) -> Option<u32> {
        let value = match digit {
            '0'..='1' if *self >= Self::Binary => digit as u32 - '0' as u32,
            '0'..='7' if *self >= Self::Octal => digit as u32 - '0' as u32,
            '0'..='9' if *self >= Self::Decimal => digit as u32 - '0' as u32,
            'a'..='f' if *self == Self::Hexadecimal => 10 + digit as u32 - 'a' as u32,
            'A'..='F' if *self == Self::Hexadecimal => 10 + digit as u32 - 'A' as u32,
            _ => return None,
        };

        Some(value)
    }
}

fn resolve_suffix(suffix: &str) -> LiteralResult<TyId> {
    let ty_id = match suffix {
        "" => tys::infer_number,
        "i8" => tys::i8,
        "i16" => tys::i16,
        "i32" => tys::i32,
        "i64" => tys::i64,
        "i128" => tys::i128,
        "isize" | "i" => tys::isize,
        "u8" => tys::u8,
        "u16" => tys::u16,
        "u32" => tys::u32,
        "u64" => tys::u64,
        "u128" => tys::u128,
        "usize" | "u" => tys::usize,
        "f32" => tys::f32,
        "f64" => tys::f64,
        _ => return Err(LiteralError::SuffixUnknown),
    };

    Ok(ty_id)
}
