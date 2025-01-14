use std::env;
use std::path::{Path, PathBuf};

pub fn res_path(path: &Path) -> PathBuf {
    env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("res"))
        .join(path)
}
