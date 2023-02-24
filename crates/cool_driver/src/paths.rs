use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum ModulePathsError {
    NoPathFound,
    MultiplePathsFound,
    BadExtension,
}

#[derive(Clone, Debug)]
pub struct ModulePaths {
    pub module_path: PathBuf,
    pub child_module_dir: PathBuf,
}

impl ModulePaths {
    pub fn for_root(module_path: &Path) -> Result<ModulePaths, ModulePathsError> {
        if module_path.extension().filter(|&ext| ext == "cl").is_none() {
            return Err(ModulePathsError::BadExtension);
        }

        if !module_path.exists() {
            return Err(ModulePathsError::NoPathFound);
        }

        let child_module_dir = module_path.parent().ok_or(ModulePathsError::NoPathFound)?;

        Ok(Self {
            module_path: module_path.to_path_buf(),
            child_module_dir: child_module_dir.to_path_buf(),
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

    fn for_child_same_dir(child_dir: &Path, module_name: &str) -> Option<ModulePaths> {
        let mut module_path = PathBuf::new();
        module_path.push(child_dir);
        module_path.push(format!("{module_name}.cl"));

        if !module_path.exists() {
            return None;
        }

        let mut child_module_dir = PathBuf::new();
        child_module_dir.push(child_dir);
        child_module_dir.push(module_name);

        Some(ModulePaths {
            module_path,
            child_module_dir,
        })
    }

    fn for_child_separate_dir(child_dir: &Path, module_name: &str) -> Option<ModulePaths> {
        let mut module_path = PathBuf::new();
        module_path.push(child_dir);
        module_path.push(module_name);
        module_path.push("@module.cl");

        if !module_path.exists() {
            return None;
        }

        let mut child_module_dir = PathBuf::new();
        child_module_dir.push(child_dir);
        child_module_dir.push(module_name);

        Some(ModulePaths {
            module_path,
            child_module_dir,
        })
    }
}

/*
Module {
    parent: Module?

    items:
        private
        public

    uses:
        private
        public
}

*/
