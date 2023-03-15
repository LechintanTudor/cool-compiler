use crate::AstGenerator;
use cool_lexer::symbols::Symbol;
use cool_parser::FnItem;
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct FnAst {
    pub ty_id: TyId,
    pub args: Vec<FnArgAst>,
    pub ret_ty_id: TyId,
}

#[derive(Clone, Debug)]
pub struct FnArgAst {
    pub is_mutable: bool,
    pub ident: Symbol,
    pub ty_id: TyId,
}

impl AstGenerator<'_> {
    pub fn generate_fn(&mut self, function: &FnItem) -> FnAst {
        let mut args = Vec::<FnArgAst>::new();

        for arg in function.arg_list.args.iter() {
            let arg_ty_id = self.resolve_ty(&arg.ty).unwrap();

            args.push(FnArgAst {
                is_mutable: arg.is_mutable,
                ident: arg.ident,
                ty_id: arg_ty_id,
            });
        }

        let ret_ty_id = match &function.return_ty {
            Some(return_ty) => self.resolve_ty(return_ty).unwrap(),
            None => tys::UNIT,
        };

        let arg_ty_ids = args.iter().map(|arg| arg.ty_id);
        let fn_ty_id = self.tys.mk_fn(arg_ty_ids, ret_ty_id);

        FnAst {
            ty_id: fn_ty_id,
            args,
            ret_ty_id,
        }
    }
}
