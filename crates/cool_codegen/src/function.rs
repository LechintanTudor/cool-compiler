use crate::{basic_value_from_any_value_enum, CodeGenerator};
use cool_ast::{ExternFnAst, FnAst};
use cool_lexer::symbols::sym;
use cool_resolve::ItemPath;
use inkwell::values::AnyValue;

impl CodeGenerator<'_> {
    pub fn add_extern_fn(&mut self, extern_fn_ast: &ExternFnAst) {
        let fn_name = self
            .resolve
            .get_path_by_item_id(extern_fn_ast.item_id)
            .last()
            .as_str();

        let fn_ty = self.tys[extern_fn_ast.ty_id].into_function_type();
        let binding_id = self.resolve[extern_fn_ast.item_id].as_binding_id().unwrap();
        let fn_value = self.module.add_function(fn_name, fn_ty, None);

        debug_assert!(!self.fns.contains_key(&extern_fn_ast.item_id));
        debug_assert!(!self.bindings.contains_key(&binding_id));

        self.fns.insert(extern_fn_ast.item_id, fn_value);
        self.bindings
            .insert(binding_id, fn_value.as_any_value_enum());
    }

    pub fn add_fn(&mut self, fn_ast: &FnAst) {
        let fn_name = mangle_item_path(self.resolve.get_path_by_item_id(fn_ast.item_id));
        let fn_ty = self.tys[fn_ast.ty_id].into_function_type();
        let binding_id = self.resolve[fn_ast.item_id].as_binding_id().unwrap();
        let fn_value = self.module.add_function(&fn_name, fn_ty, None);

        debug_assert!(!self.fns.contains_key(&fn_ast.item_id));
        debug_assert!(!self.bindings.contains_key(&binding_id));

        self.fns.insert(fn_ast.item_id, fn_value);
        self.bindings
            .insert(binding_id, fn_value.as_any_value_enum());
    }

    pub fn gen_fn(&mut self, fn_ast: &FnAst) {
        let fn_value = self.fns[&fn_ast.item_id];
        let entry = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry);

        if let Some(ret_value) = self.gen_block_expr(&fn_ast.body) {
            if let Some(ret_value) = basic_value_from_any_value_enum(ret_value) {
                self.builder.build_return(Some(&ret_value));
            } else {
                self.builder.build_return(None);
            }
        } else {
            self.builder.build_return(None);
        }
    }
}

fn mangle_item_path<'a, P>(path: P) -> String
where
    P: Into<ItemPath<'a>>,
{
    let path: ItemPath = path.into();

    if path.last() == sym::MAIN {
        return "main".to_owned();
    }

    let path = path.as_symbol_slice();

    let Some((&first, others)) = path.split_first() else {
        return String::new();
    };

    let mut result = String::from(first.as_str());

    for other in others {
        result.push_str("__");
        result.push_str(other.as_str());
    }

    result
}
