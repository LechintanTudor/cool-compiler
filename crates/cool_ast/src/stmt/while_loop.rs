use crate::{AstGenerator, AstResult, CondBlockAst};
use cool_parser::WhileLoop;
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct WhileLoopAst {
    pub span: Span,
    pub block: Box<CondBlockAst>,
}

impl Section for WhileLoopAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_while_loop(
        &mut self,
        frame_id: FrameId,
        stmt: &WhileLoop,
    ) -> AstResult<WhileLoopAst> {
        self.push_block_ty_id(self.tys().unit);
        let block = self.gen_cond_block(frame_id, self.tys().unit, &stmt.block)?;
        self.pop_block_ty_id();

        Ok(WhileLoopAst {
            span: stmt.span,
            block: Box::new(block),
        })
    }
}
