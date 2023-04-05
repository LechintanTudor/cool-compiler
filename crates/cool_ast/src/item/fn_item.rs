use crate::expr::BlockExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_parser::FnItem;
use cool_resolve::resolve::{BindingId, FrameId, ModuleId};
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct FnItemAst {
    pub ty_id: TyId,
    pub frame_id: FrameId,
    pub bindings: Vec<BindingId>,
    pub body: BlockExprAst,
}

impl ResolveAst for FnItemAst {
    fn resolve(&self, ast: &mut AstGenerator, _expected_ty: TyId) -> SemanticResult<TyId> {
        self.body.resolve(ast, tys::INFERRED)?;
        Ok(self.ty_id)
    }
}

impl AstGenerator<'_> {
    // TODO: Use item type to infer types
    pub fn gen_fn(&mut self, module_id: ModuleId, fn_item: &FnItem) -> FnItemAst {
        let param_ty_ids = fn_item
            .prototype
            .param_list
            .params
            .iter()
            .map(|param| {
                self.resolve_parsed_ty(module_id.into(), param.ty.as_ref().unwrap())
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let return_ty_id = fn_item
            .prototype
            .return_ty
            .as_ref()
            .map(|ty| self.resolve_parsed_ty(module_id.into(), ty).unwrap())
            .unwrap_or(tys::UNIT);

        let ty_id = self.tys.mk_fn(param_ty_ids.iter().copied(), return_ty_id);

        let frame_id = self.resolve.insert_frame(module_id.into());
        let mut bindings = Vec::new();

        for param in fn_item.prototype.param_list.params.iter() {
            let binding = self
                .resolve
                .insert_local_binding(frame_id, param.is_mutable, param.ident.symbol)
                .unwrap();

            bindings.push(binding);
        }

        let body = self.gen_block_expr(frame_id.into(), &fn_item.body);

        FnItemAst {
            ty_id,
            frame_id,
            bindings,
            body,
        }
    }
}
