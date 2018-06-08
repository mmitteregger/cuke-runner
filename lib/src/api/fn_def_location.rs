use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FnDefLocation {
    file_path: PathBuf,
    line_number: u32,
}

impl fmt::Display for FnDefLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path.display(), self.line_number)
    }
}
