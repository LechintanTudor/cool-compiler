mod version;

pub use self::version::*;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Package {
    pub package: PackageHeader,

    #[serde(default)]
    pub dependencies: AHashMap<String, Dependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PackageHeader {
    pub name: String,
    pub version: Version,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub package: Option<String>,
    pub version: Version,
}
