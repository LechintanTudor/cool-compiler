use cool_collections::{SmallVec, VecMap};
use cool_parser::ParserData;
use cool_resolve::{CrateId, ResolveContext};
use std::path::PathBuf;

#[derive(Clone, Default, Debug)]
pub struct Project {
    pub crates: VecMap<CrateId, Crate>,
    pub dependencies: VecMap<CrateId, SmallVec<CrateId, 4>>,
}

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
}

pub fn p0_parse(project: Project, context: &mut ResolveContext) -> ParserData {
    todo!()
}
