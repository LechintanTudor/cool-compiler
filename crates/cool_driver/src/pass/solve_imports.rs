use crate::pass::Project;
use cool_resolve::ResolveContext;
use std::collections::VecDeque;
use std::mem;

pub fn solve_imports(project: &mut Project, context: &mut ResolveContext) {
    let mut imports = VecDeque::from(mem::take(&mut project.imports));

    loop {
        let mut made_progress = false;

        for _ in 0..imports.len() {
            let import = imports.pop_front().unwrap();

            match context.add_import(
                import.module_id,
                import.is_exported,
                &import.path,
                import.symbol,
            ) {
                Ok(_) => made_progress = true,
                Err(_) => imports.push_back(import),
            }
        }

        if !made_progress {
            break;
        }
    }

    project.imports = Vec::from(imports);
}
