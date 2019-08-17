use proc_macro::SourceFile;
use std::path::{Path, PathBuf};

pub fn source_file_path(source_file: &SourceFile) -> PathBuf {
    let source_file_path = source_file.path();
    match source_file_path.canonicalize() {
        Ok(canonicalized_path) => canonicalized_path,
        Err(_) => source_file_path,
    }
}

pub fn path_to_str(path: &Path) -> &str {
    match path.to_str() {
        Some(path_str) => path_str,
        None => {
            panic!("Path \"{}\" cannot be losslessly converted to an UTF-8 string \
            and is thus currently not supported", path.display());
        },
    }
}
