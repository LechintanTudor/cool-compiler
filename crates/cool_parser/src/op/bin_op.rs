use crate::op::define_op;

define_op! {
    BinOp {
        // Arithmetic
        Add => "+" from Plus,
        Sub => "-" from Minus,
        Mul => "*" from Star,
        Div => "/" from Slash,
        Rem => "%" from Percent,

        // Relational
        Eq => "==" from EqEq,
        Ne => "!=" from Ne,
        Lt => "<" from Lt,
        Le => "<=" from Le,
        Gt => ">" from Gt,
        Ge => ">=" from Ge,

        // Bitwise
        And => "&" from And,
        Or => "|" from Or,
        Xor => "^" from Caret,
        Shl => "<<" from Shl,
        Shr => ">>" from Shr,

        LogicalAnd => "&&" from AndAnd,
        LogicalOr => "||" from OrOr,
    }
}
