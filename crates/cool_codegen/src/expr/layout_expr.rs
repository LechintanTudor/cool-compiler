use crate::CodeGenerator;
use cool_ast::{AlignOfExprAst, SizeOfExprAst};
use inkwell::values::IntValue;

impl<'a> CodeGenerator<'a> {
    #[inline]
    pub fn gen_size_of_expr(&mut self, expr: &SizeOfExprAst) -> IntValue<'a> {
        self.tys.isize_ty().const_int(expr.value, false)
    }

    #[inline]
    pub fn gen_align_of_expr(&mut self, expr: &AlignOfExprAst) -> IntValue<'a> {
        self.tys.isize_ty().const_int(expr.value, false)
    }
}
