use cool_collections::{SmallString, VecMap};
use cool_package::{PackageSpec, Version};
use derive_more::{Display, Error, From};
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};
use toml::de::Error as TomlError;

#[derive(Clone, Debug)]
pub struct ProjectData {
    pub crates: VecMap<ast::CrateId, CrateData>,
}

#[derive(Clone, Debug)]
pub struct CrateData {
    pub name: SmallString,
    pub version: Version,
    pub kind: CrateKind,
    pub path: PathBuf,
    pub deps: Vec<DependencyData>,
}

impl CrateData {
    #[must_use]
    pub fn entry_path(&self) -> PathBuf {
        let file_name = match self.kind {
            CrateKind::Executable => "@main.cool",
            CrateKind::Library => "@lib.cool",
        };

        let mut path = self.path.clone();
        path.push("src");
        path.push(file_name);
        path
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CrateKind {
    Executable,
    Library,
}

#[derive(Clone, Debug)]
pub struct DependencyData {
    pub crate_id: ast::CrateId,
    pub mount_name: SmallString,
}

#[derive(Error, Debug, Display)]
#[display("Error in file {}.\n{}", self.path.display(), self.kind)]
pub struct ProjectError {
    pub path: PathBuf,
    pub kind: ProjectErrorKind,
}

#[derive(From, Debug, Display)]
pub enum ProjectErrorKind {
    Io(IoError),

    Toml(TomlError),

    NoEntry,

    TooManyEntries,

    #[display("Failed to solve dependency: {:#?}", _0)]
    UnsolvedDependency(DependencyToSolve),

    #[display("Project has circular dependencies")]
    CircularDependency,
}

#[derive(Clone, Debug)]
pub struct DependencyToSolve {
    crate_id: ast::CrateId,
    dep_name: SmallString,
    mount_name: SmallString,
}

pub fn read_project(
    crate_paths: &[PathBuf],
) -> Result<ProjectData, (ProjectData, Vec<ProjectError>)> {
    let mut crates = VecMap::<ast::CrateId, CrateData>::default();
    let mut deps = Vec::<DependencyToSolve>::new();
    let mut errors = Vec::<ProjectError>::new();

    let mut file_buf = String::new();
    let mut path_buf = PathBuf::new();

    for crate_path in crate_paths {
        path_buf.clear();
        path_buf.push(crate_path);
        path_buf.push("package.toml");

        let mut package_file = match File::open(&path_buf) {
            Ok(package_file) => package_file,
            Err(error) => {
                errors.push(ProjectError {
                    path: path_buf.clone(),
                    kind: error.into(),
                });
                continue;
            }
        };

        file_buf.clear();
        if let Err(error) = package_file.read_to_string(&mut file_buf) {
            errors.push(ProjectError {
                path: path_buf.clone(),
                kind: error.into(),
            });
            continue;
        }

        let package = match toml::from_str::<PackageSpec>(&file_buf) {
            Ok(package) => package,
            Err(error) => {
                errors.push(ProjectError {
                    path: path_buf.clone(),
                    kind: error.into(),
                });
                continue;
            }
        };

        let kind = match get_crate_kind(&crate_path) {
            Ok(kind) => kind,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        let crate_id = crates.push(CrateData {
            name: package.package.name,
            version: package.package.version,
            kind,
            path: crate_path.clone(),
            deps: Vec::new(),
        });

        for (dep_name, dep) in package.dependencies {
            deps.push(DependencyToSolve {
                crate_id,
                dep_name: dep.name.unwrap_or(dep_name.clone()),
                mount_name: dep_name,
            });
        }
    }

    for dep in deps {
        let dep_id = crates
            .iter_with_index()
            .find(|(_, c)| c.name == dep.dep_name)
            .map(|(i, _)| i);

        match dep_id {
            Some(dep_id) => {
                crates[dep.crate_id].deps.push(DependencyData {
                    crate_id: dep_id,
                    mount_name: dep.mount_name,
                })
            }
            None => {
                errors.push(ProjectError {
                    path: crates[dep.crate_id].path.clone(),
                    kind: ProjectErrorKind::UnsolvedDependency(dep),
                })
            }
        }
    }

    if has_any_circular_dependency(&crates) {
        errors.push(ProjectError {
            path: PathBuf::default(),
            kind: ProjectErrorKind::CircularDependency,
        });
    }

    let data = ProjectData { crates };

    if !errors.is_empty() {
        return Err((data, errors));
    }

    Ok(data)
}

fn get_crate_kind(path: &Path) -> Result<CrateKind, ProjectError> {
    let mut main_path = PathBuf::default();
    main_path.push(path);
    main_path.push("src/@main.cool");

    let mut lib_path = PathBuf::default();
    lib_path.push(path);
    lib_path.push("src/@lib.cool");

    match (main_path.exists(), lib_path.exists()) {
        (true, false) => Ok(CrateKind::Executable),
        (false, true) => Ok(CrateKind::Library),
        (false, false) => {
            Err(ProjectError {
                path: path.into(),
                kind: ProjectErrorKind::NoEntry,
            })
        }
        (true, true) => {
            Err(ProjectError {
                path: path.into(),
                kind: ProjectErrorKind::TooManyEntries,
            })
        }
    }
}

fn has_any_circular_dependency(crates: &[CrateData]) -> bool {
    let mut visited = Vec::from_iter((0..crates.len()).map(|_| false));
    let mut current = Vec::from_iter((0..crates.len()).map(|_| false));

    for i in 0..crates.len() {
        if visited[i] {
            continue;
        }

        if has_circular_dependency(crates, &mut visited, &mut current, i) {
            return true;
        }
    }

    false
}

fn has_circular_dependency(
    crates: &[CrateData],
    visited: &mut [bool],
    current: &mut [bool],
    crate_id: usize,
) -> bool {
    visited[crate_id] = true;
    current[crate_id] = true;

    for dep in crates[crate_id].deps.iter() {
        let dep_id = dep.crate_id.get() as usize;

        if !visited[dep_id] {
            if has_circular_dependency(crates, visited, current, dep_id) {
                return true;
            }
        } else if current[dep_id] {
            return true;
        }
    }

    current[crate_id] = false;
    false
}
