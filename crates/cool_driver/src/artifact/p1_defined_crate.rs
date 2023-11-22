use crate::{ParsedFn, SourceFile, SourceId};
use cool_collections::VecMap;

#[derive(Clone, Debug)]
pub struct DefinedCrate {
    pub files: VecMap<SourceId, SourceFile>,
    pub fns: Vec<ParsedFn>,
}
