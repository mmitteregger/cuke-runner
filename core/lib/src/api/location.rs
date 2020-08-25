use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GlueCodeLocation {
    pub(crate) file_path: PathBuf,
    pub(crate) line_number: usize,
}

impl GlueCodeLocation {
    /// Returns the relative path to the glue code file.
    pub fn file_path(&self) -> &Path {
        self.file_path.as_path()
    }

    /// Returns the line number in the glue code file.
    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl fmt::Display for GlueCodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path.display(), self.line_number)
    }
}
