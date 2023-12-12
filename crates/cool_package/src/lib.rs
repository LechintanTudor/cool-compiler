mod project;
mod version;

pub use self::project::*;
pub use self::version::*;

use cool_collections::SmallString;

#[derive(Clone, Debug)]
pub struct Package {
    pub header: PackageHeader,
    pub dependencies: Vec<PackageHeader>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackageHeader {
    pub name: SmallString,
    pub version: Version,
}
