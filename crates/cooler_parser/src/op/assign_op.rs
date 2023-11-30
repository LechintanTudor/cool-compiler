use crate::op::define_op;

define_op! {
    AssignOp {
        Eq => "=" from Eq,
        Add => "+= " from PlusEq,
        Sub => "-=" from MinusEq,
        Mul => "*=" from StarEq,
        Div => "/=" from SlashEq,
        Rem => "%=" from PerecentEq,
        Or => "|=" from OrEq,
        And => "&=" from AndEq,
        Xor => "^=" from CaretEq,
        Shl => "<<=" from ShlEq,
        Shr => ">>=" from ShrEq,
    }
}
