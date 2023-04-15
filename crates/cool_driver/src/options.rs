use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub crate_name: String,
    pub crate_root_file: PathBuf,
}
