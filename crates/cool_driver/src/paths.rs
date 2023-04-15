use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum ModulePathsError {
    #[error("no module path found")]
    NoPathFound,

    #[error("multiple module paths found")]
    MultiplePathsFound,

    #[error("bad source file extension")]
    BadExtension,
}

#[derive(Clone, Debug)]
pub struct ModulePaths {
    pub path: PathBuf,
    pub child_dir: PathBuf,
}

impl ModulePaths {
    pub fn for_root(path: &Path) -> Result<ModulePaths, ModulePathsError> {
        if path.extension().filter(|&ext| ext == "cl").is_none() {
            return Err(ModulePathsError::BadExtension);
        }

        if !path.exists() {
            return Err(ModulePathsError::NoPathFound);
        }

        let child_dir = path.parent().ok_or(ModulePathsError::NoPathFound)?;

        Ok(Self {
            path: path.to_path_buf(),
            child_dir: child_dir.to_path_buf(),
        })
    }

    pub fn for_child(child_dir: &Path, module_name: &str) -> Result<ModulePaths, ModulePathsError> {
        let paths1 = Self::for_child_same_dir(child_dir, module_name);
        let paths2 = Self::for_child_separate_dir(child_dir, module_name);

        match (paths1, paths2) {
            (Some(paths1), None) => Ok(paths1),
            (None, Some(paths2)) => Ok(paths2),
            (None, None) => Err(ModulePathsError::NoPathFound),
            (Some(_), Some(_)) => Err(ModulePathsError::MultiplePathsFound),
        }
    }

    fn for_child_same_dir(parent_child_dir: &Path, module_name: &str) -> Option<ModulePaths> {
        let mut path = PathBuf::new();
        path.push(parent_child_dir);
        path.push(format!("{module_name}.cl"));

        if !path.exists() {
            return None;
        }

        let mut child_dir = PathBuf::new();
        child_dir.push(parent_child_dir);
        child_dir.push(module_name);

        Some(ModulePaths { path, child_dir })
    }

    fn for_child_separate_dir(parent_child_dir: &Path, module_name: &str) -> Option<ModulePaths> {
        let mut path = PathBuf::new();
        path.push(parent_child_dir);
        path.push(module_name);
        path.push("@module.cl");

        if !path.exists() {
            return None;
        }

        let mut child_dir = PathBuf::new();
        child_dir.push(parent_child_dir);
        child_dir.push(module_name);

        Some(ModulePaths { path, child_dir })
    }
}
