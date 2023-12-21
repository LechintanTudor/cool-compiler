mod version;

pub use self::version::*;

use ahash::AHashMap;
use cool_collections::SmallString;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkspaceSpec {
    pub workspace: Workspace,

    #[serde(default)]
    pub dependencies: AHashMap<SmallString, Dependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub members: Vec<SmallString>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackageSpec {
    pub package: Package,

    #[serde(default)]
    pub dependencies: AHashMap<SmallString, Dependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    pub name: SmallString,
    pub version: Version,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub name: Option<SmallString>,

    pub version: Option<Version>,

    pub path: Option<PathBuf>,

    #[serde(default)]
    pub workspace: bool,
}
