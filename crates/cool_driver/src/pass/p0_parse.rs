use cool_lexer::Symbol;
use cool_parser::ParserData;
use cool_resolve::{CrateId, ResolveContext};
use std::path::PathBuf;

#[derive(Clone, Default, Debug)]
pub struct Project {
    pub crates: Vec<Crate>,
}

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
    pub deps: Vec<usize>,
}

pub fn p0_parse(project: &Project, context: &mut ResolveContext) -> ParserData {
    let mut crates = Vec::<(CrateId, &Crate)>::new();

    // Add all crates
    for project_crate in &project.crates {
        let name = Symbol::insert(&project_crate.name);
        let crate_id = context.add_crate(name);
        crates.push((crate_id, project_crate));
    }

    // Mount all dependencies
    for (crate_id, project_crate) in &crates {
        for dep in &project_crate.deps {
            let (dep_id, dep) = crates[*dep];
            let dep_name = Symbol::insert(&dep.name);
            context.add_dep(*crate_id, dep_name, dep_id).unwrap();
        }
    }

    ParserData::default()
}
