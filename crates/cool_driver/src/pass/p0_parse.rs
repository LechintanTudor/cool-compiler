use cool_collections::{define_index_newtype, VecMap};
use cool_parser::{ItemId, ParserData};
use std::path::PathBuf;

define_index_newtype!(CrateId);

#[derive(Clone, Debug)]
pub struct CrateData {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone, Default, Debug)]
pub struct Project {
    pub crates: VecMap<CrateId, Crate>,
}

#[derive(Clone, Default, Debug)]
pub struct Crate {
    pub items: Vec<ItemId>,
}

pub fn p0_parse(data: &mut ParserData, crates: &[CrateData]) -> Project {
    for crate_data in crates {
        // Empty
    }

    todo!()
}
