use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct CodeLocation {
    pub file_path: &'static str,
    pub line_number: usize,
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path, self.line_number)
    }
}
