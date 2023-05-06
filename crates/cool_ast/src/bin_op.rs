use crate::TyMismatch;
use cool_parser::{ArithmeticBinOp, BitwiseBinOp, ComparisonBinOp};
use cool_resolve::{tys, TyId};
use derive_more::From;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NumberKind {
    Sint,
    Uint,
    Float,
}

impl TryFrom<TyId> for NumberKind {
    type Error = TyMismatch;

    fn try_from(ty_id: TyId) -> Result<Self, Self::Error> {
        let op_kind = if ty_id.is_signed_int() {
            Self::Sint
        } else if ty_id.is_unsigned_int() {
            Self::Uint
        } else if ty_id.is_float() {
            Self::Float
        } else {
            return Err(TyMismatch {
                found: ty_id,
                expected: tys::INFERRED_NUMBER,
            });
        };

        Ok(op_kind)
    }
}

#[derive(Clone, Copy, From, PartialEq, Eq, Hash, Debug)]
pub enum BinOpAst {
    Arithmetic(ArithmeticBinOpAst),
    Comparison(ComparisonBinOpAst),
    Bitwise(BitwiseBinOpAst),
    Logical(LogicalBinOpAst),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ArithmeticBinOpAst {
    IntAdd,
    IntSub,
    IntMul,
    UintDiv,
    SintDiv,
    UintRem,
    SintRem,
    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
    FloatRem,
}

impl ArithmeticBinOpAst {
    pub fn new(bin_op: ArithmeticBinOp, number_kind: NumberKind) -> Self {
        match bin_op {
            ArithmeticBinOp::Add => {
                match number_kind {
                    NumberKind::Sint | NumberKind::Uint => Self::IntAdd,
                    NumberKind::Float => Self::FloatAdd,
                }
            }
            ArithmeticBinOp::Sub => {
                match number_kind {
                    NumberKind::Sint | NumberKind::Uint => Self::IntSub,
                    NumberKind::Float => Self::FloatSub,
                }
            }
            ArithmeticBinOp::Mul => {
                match number_kind {
                    NumberKind::Sint | NumberKind::Uint => Self::IntMul,
                    NumberKind::Float => Self::FloatMul,
                }
            }
            ArithmeticBinOp::Div => {
                match number_kind {
                    NumberKind::Sint => Self::SintDiv,
                    NumberKind::Uint => Self::UintDiv,
                    NumberKind::Float => Self::FloatDiv,
                }
            }
            ArithmeticBinOp::Rem => {
                match number_kind {
                    NumberKind::Sint => Self::SintRem,
                    NumberKind::Uint => Self::UintRem,
                    NumberKind::Float => Self::FloatRem,
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ComparisonBinOpAst {
    IntEq,
    IntNe,
    UintLt,
    SintLt,
    UintLe,
    SintLe,
    UintGt,
    SintGt,
    UintGe,
    SintGe,
    FloatEq,
    FloatNe,
    FloatLt,
    FloatLe,
    FloatGt,
    FloatGe,
}

impl ComparisonBinOpAst {
    pub fn new(bin_op: ComparisonBinOp, number_kind: NumberKind) -> Self {
        match bin_op {
            ComparisonBinOp::Eq => {
                match number_kind {
                    NumberKind::Sint | NumberKind::Uint => ComparisonBinOpAst::IntEq,
                    NumberKind::Float => ComparisonBinOpAst::FloatEq,
                }
            }
            ComparisonBinOp::Ne => {
                match number_kind {
                    NumberKind::Sint | NumberKind::Uint => ComparisonBinOpAst::IntNe,
                    NumberKind::Float => ComparisonBinOpAst::FloatNe,
                }
            }
            ComparisonBinOp::Lt => {
                match number_kind {
                    NumberKind::Sint => ComparisonBinOpAst::SintLt,
                    NumberKind::Uint => ComparisonBinOpAst::UintLt,
                    NumberKind::Float => ComparisonBinOpAst::FloatLt,
                }
            }
            ComparisonBinOp::Le => {
                match number_kind {
                    NumberKind::Sint => ComparisonBinOpAst::SintLe,
                    NumberKind::Uint => ComparisonBinOpAst::UintLe,
                    NumberKind::Float => ComparisonBinOpAst::FloatLe,
                }
            }
            ComparisonBinOp::Gt => {
                match number_kind {
                    NumberKind::Sint => ComparisonBinOpAst::SintGe,
                    NumberKind::Uint => ComparisonBinOpAst::UintGe,
                    NumberKind::Float => ComparisonBinOpAst::FloatGe,
                }
            }
            ComparisonBinOp::Ge => {
                match number_kind {
                    NumberKind::Sint => ComparisonBinOpAst::SintGe,
                    NumberKind::Uint => ComparisonBinOpAst::UintGe,
                    NumberKind::Float => ComparisonBinOpAst::FloatGe,
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BitwiseBinOpAst {
    And,
    Or,
    Xor,
    Shl,
    Shr,
    SignExtendShr,
}

impl BitwiseBinOpAst {
    pub fn new(bin_op: BitwiseBinOp, number_kind: NumberKind) -> Self {
        match bin_op {
            BitwiseBinOp::And => Self::And,
            BitwiseBinOp::Or => Self::Or,
            BitwiseBinOp::Xor => Self::Xor,
            BitwiseBinOp::Shl => Self::Shl,
            BitwiseBinOp::Shr => {
                match number_kind {
                    NumberKind::Sint => Self::SignExtendShr,
                    NumberKind::Uint => Self::Shr,
                    NumberKind::Float => panic!("bitwise operations don't work on floats"),
                }
            }
        }
    }

    #[inline]
    pub fn is_shift(&self) -> bool {
        matches!(self, Self::Shl | Self::Shr | Self::SignExtendShr)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogicalBinOpAst {
    And,
    Or,
}
