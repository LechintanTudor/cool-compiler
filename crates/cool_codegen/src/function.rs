use crate::{mangle_item_path, BuilderExt, CodeGenerator, FnState, LoadedValue, Value};
use cool_ast::{ExternFnAst, FnAst};
use inkwell::values::BasicValue;

impl CodeGenerator<'_> {
    pub fn add_extern_fn(&mut self, extern_fn_ast: &ExternFnAst) {
        let fn_name = self
            .resolve
            .get_path_by_item_id(extern_fn_ast.item_id)
            .last()
            .as_str();

        let fn_ty = self.tys.get_fn_ty(extern_fn_ast.ty_id);
        let binding_id = self.resolve[extern_fn_ast.item_id].as_binding_id().unwrap();
        let fn_value = self.module.add_function(fn_name, fn_ty, None);

        debug_assert!(!self.bindings.contains_key(&binding_id));
        self.bindings.insert(binding_id, fn_value.into());
    }

    pub fn add_fn(&mut self, fn_ast: &FnAst) {
        let fn_name = mangle_item_path(self.resolve.get_path_by_item_id(fn_ast.item_id));
        let fn_ty = self.tys.get_fn_ty(fn_ast.ty_id);
        let binding_id = self.resolve[fn_ast.item_id].as_binding_id().unwrap();
        let fn_value = self.module.add_function(&fn_name, fn_ty, None);

        debug_assert!(!self.bindings.contains_key(&binding_id));
        self.bindings.insert(binding_id, fn_value.into());
    }

    pub fn gen_fn(&mut self, fn_ast: &FnAst) {
        let binding_id = self.resolve[fn_ast.item_id].as_binding_id().unwrap();
        let fn_value = self.bindings[&binding_id].into_function_value();
        self.fn_stack.push(FnState::new(fn_value));

        let entry_block = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry_block);

        let mut param_value_iter = fn_value.get_param_iter();

        for &binding_id in fn_ast.binding_ids.iter() {
            let param = self.resolve[binding_id];

            let param_value = if self.resolve.is_ty_zero_sized(param.ty_id) {
                Value::Void
            } else {
                let value = param_value_iter.next().unwrap().as_basic_value_enum();
                let ptr = self.util_gen_named_init(value, param.symbol.as_str());
                let ty = value.get_type();
                Value::memory(ptr, ty)
            };

            debug_assert!(!self.bindings.contains_key(&binding_id));
            self.bindings.insert(binding_id, param_value);
        }

        let ret_value = self.gen_block_expr(&fn_ast.body);

        if !self.builder.current_block_diverges() {
            let ret_value = match &ret_value {
                LoadedValue::Void => None,
                LoadedValue::Register(value) => Some(value as &dyn BasicValue),
            };

            self.builder.build_return(ret_value);
        }

        self.pass_manager.run_on(&fn_value);
        self.fn_stack.pop();
    }
}
