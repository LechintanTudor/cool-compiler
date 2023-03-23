use crate::expr::BlockExprAst;
use crate::AstGenerator;
use cool_parser::FnItem;
use cool_resolve::resolve::{FrameId, ModuleId};
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct FnItemAst {
    pub ty_id: TyId,
    pub frame_id: FrameId,
    pub body: BlockExprAst,
}

impl AstGenerator<'_> {
    pub fn generate_fn(&mut self, module_id: ModuleId, fn_item: &FnItem) -> FnItemAst {
        let args_ty_ids = fn_item
            .arg_list
            .args
            .iter()
            .map(|arg| self.resolve_ty(module_id.into(), &arg.ty).unwrap())
            .collect::<Vec<_>>();

        let ret_ty_id = fn_item
            .return_ty
            .as_ref()
            .map(|ty| self.resolve_ty(module_id.into(), &ty).unwrap())
            .unwrap_or(tys::UNIT);

        let ty_id = self.tys.mk_fn(args_ty_ids.iter().copied(), ret_ty_id);

        let frame_id = self.resolve.insert_frame(module_id.into());
        for (arg, _arg_ty_id) in fn_item
            .arg_list
            .args
            .iter()
            .zip(args_ty_ids.iter().copied())
        {
            self.resolve
                .insert_local_binding(frame_id, arg.is_mutable, arg.ident)
                .unwrap();
        }

        let body = self.generate_block_expr(frame_id.into(), &fn_item.body);

        FnItemAst {
            ty_id,
            frame_id,
            body,
        }
    }
}
