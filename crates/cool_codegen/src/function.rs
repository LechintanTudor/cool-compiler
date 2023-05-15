use crate::{AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::{ExternFnAst, FnAst};
use cool_collections::SmallString;
use cool_lexer::symbols::sym;
use cool_resolve::ItemPath;
use inkwell::types::BasicType;
use inkwell::values::{AnyValue, BasicValue};

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
            .insert(binding_id, fn_value.as_any_value_enum().into());
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
            .insert(binding_id, fn_value.as_any_value_enum().into());
    }

    pub fn gen_fn(&mut self, fn_ast: &FnAst) {
        let fn_value = self.fns[&fn_ast.item_id];
        self.fn_value = Some(fn_value);

        let entry_block = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry_block);

        let mut param_value_iter = fn_value.get_param_iter();

        for &binding_id in fn_ast.binding_ids.iter() {
            let param = self.resolve[binding_id];

            let param_value = if self.resolve.is_ty_id_zst(param.ty_id) {
                Value::Void
            } else {
                let value = param_value_iter.next().unwrap().as_basic_value_enum();
                let pointer = self.util_gen_alloca(value, param.symbol.as_str());
                let ty = value.get_type().as_basic_type_enum();
                Value::Lvalue { pointer, ty }
            };

            debug_assert!(!self.bindings.contains_key(&binding_id));
            self.bindings.insert(binding_id, param_value);
        }

        let ret_value = self.gen_block_expr(&fn_ast.body);
        let ret_value = self
            .gen_loaded_value(ret_value)
            .and_then(|ret_value| ret_value.try_into_basic_value());

        let ret_value = ret_value
            .as_ref()
            .map(|ret_value| ret_value as &dyn BasicValue);

        self.builder.build_return(ret_value);
        self.pass_manager.run_on(&fn_value);

        self.fn_value = None;
        self.last_alloca = None;
    }
}

fn mangle_item_path<'a, P>(path: P) -> SmallString
where
    P: Into<ItemPath<'a>>,
{
    let path: ItemPath = path.into();

    if path.last() == sym::MAIN {
        return SmallString::from("main");
    }

    let path = path.as_symbol_slice();

    let Some((&first, others)) = path.split_first() else {
        return SmallString::new();
    };

    let mut result = SmallString::from(first.as_str());

    for other in others {
        result.push_str("__");
        result.push_str(other.as_str());
    }

    result
}
