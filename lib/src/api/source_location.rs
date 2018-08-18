use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceCodeLocation {
    pub file_path: String,
    pub line_number: u32,
}

impl fmt::Display for SourceCodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path, self.line_number)
    }
}
