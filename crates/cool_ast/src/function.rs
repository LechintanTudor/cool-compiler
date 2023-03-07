use crate::AstGenerator;
use cool_lexer::symbols::Symbol;
use cool_parser::item::FnItem;
use cool_parser::path::SymbolPath;
use cool_parser::ty::Ty;
use cool_resolve::item::ItemPathBuf;
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct FnAst {
    pub ty: TyId,
    pub args: Vec<FnArgAst>,
}

#[derive(Clone, Debug)]
pub struct FnArgAst {
    pub is_mutable: bool,
    pub ident: Symbol,
}

impl AstGenerator<'_> {
    pub fn generate_fn(&mut self, function: &FnItem) -> FnAst {
        let mut arg_ty_ids = Vec::<TyId>::new();
        let mut args = Vec::<FnArgAst>::new();

        for arg in function.arg_list.args.iter() {
            let ty_id = match &arg.ty {
                Ty::Path(path) => {
                    let path = symbol_path_to_item_path(path);
                    self.get_ty_id_by_path(&path).expect("ty not found")
                }
                Ty::Tuple(_tuple_ty) => todo!(),
            };

            arg_ty_ids.push(ty_id);
            args.push(FnArgAst {
                is_mutable: arg.is_mutable,
                ident: arg.ident,
            });
        }

        let ty = self.tys.mk_fn(arg_ty_ids, tys::U32);

        FnAst { ty, args }
    }
}

fn symbol_path_to_item_path(symbol_path: &SymbolPath) -> ItemPathBuf {
    ItemPathBuf::from_iter(symbol_path.idents.iter().map(|ident| ident.symbol))
}
