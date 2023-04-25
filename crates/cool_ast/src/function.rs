use crate::{AstGenerator, AstResult, BlockExprAst};
use cool_parser::FnExpr;
use cool_resolve::{BindingId, FrameId, ItemId, ModuleId, TyId};
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct ExternFnAst {
    pub item_id: ItemId,
    pub ty_id: TyId,
}

#[derive(Clone, Debug)]
pub struct FnAst {
    pub item_id: ItemId,
    pub ty_id: TyId,
    pub frame_id: FrameId,
    pub binding_ids: SmallVec<[BindingId; 4]>,
    pub body: BlockExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_extern_fn(&mut self, item_id: ItemId, ty_id: TyId) -> AstResult<ExternFnAst> {
        Ok(ExternFnAst { item_id, ty_id })
    }

    pub fn gen_fn(
        &mut self,
        item_id: ItemId,
        module_id: ModuleId,
        ty_id: TyId,
        fn_expr: &FnExpr,
    ) -> AstResult<FnAst> {
        let frame_id = self.resolve.add_frame(module_id.into());
        let fn_ty = self.resolve[ty_id].as_fn_ty().unwrap().clone();

        let param_ty_iter = fn_expr
            .prototype
            .param_list
            .params
            .iter()
            .zip(fn_ty.params.iter());

        let mut binding_ids = SmallVec::new();

        for (param, param_ty_id) in param_ty_iter {
            let binding_id = self
                .resolve
                .insert_local_binding(
                    frame_id,
                    param.is_mutable,
                    param.ident.symbol,
                    Some(*param_ty_id),
                )
                .unwrap();

            binding_ids.push(binding_id);
        }

        Ok(FnAst {
            item_id,
            ty_id,
            frame_id,
            binding_ids,
            body: todo!(),
        })
    }
}
