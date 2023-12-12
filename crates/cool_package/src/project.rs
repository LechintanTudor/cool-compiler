use std::collections::VecDeque;

use crate::{Package, PackageHeader};
use ahash::{AHashMap, AHashSet};
use cool_collections::{define_index_newtype, SmallVec, VecMap};

define_index_newtype!(PackageId);

#[derive(Clone, Debug)]
pub struct Project {
    pub packages: VecMap<PackageId, ProjectPackage>,
}

impl Project {
    #[inline]
    pub fn builder() -> ProjectBuilder {
        Default::default()
    }
}

#[derive(Clone, Debug)]
pub struct ProjectPackage {
    pub header: PackageHeader,
    pub dependencies: SmallVec<PackageId, 4>,
}

impl From<PackageHeader> for ProjectPackage {
    #[inline]
    fn from(header: PackageHeader) -> Self {
        Self {
            header,
            dependencies: SmallVec::new(),
        }
    }
}

#[derive(Default)]
pub struct ProjectBuilder {
    packages: VecMap<PackageId, ProjectPackage>,
    package_ids: AHashMap<PackageHeader, PackageId>,
    solved_packages: AHashSet<PackageHeader>,
}

impl ProjectBuilder {
    pub fn add_package(
        &mut self,
        package: Package,
        missing_packages: &mut VecDeque<PackageHeader>,
    ) {
        if self.solved_packages.contains(&package.header) {
            return;
        }

        let mut dependencies = SmallVec::new();

        for dependency in package.dependencies {
            let dependency_id = match self.package_ids.get(&dependency) {
                Some(dependency_id) => *dependency_id,
                None => {
                    missing_packages.push_back(dependency.clone());
                    self.packages.push(dependency.into())
                }
            };

            dependencies.push(dependency_id);
        }

        match self.package_ids.get(&package.header) {
            Some(package_id) => {
                self.packages[*package_id].dependencies = dependencies;
            }
            None => {
                self.packages.push(ProjectPackage {
                    header: package.header.clone(),
                    dependencies,
                });
            }
        };

        self.solved_packages.insert(package.header);
    }

    pub fn build(self) -> Project {
        Project {
            packages: self.packages,
        }
    }
}
