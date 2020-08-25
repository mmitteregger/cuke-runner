use std::fmt;

#[derive(Debug, Clone)]
pub struct StaticGlueCodeLocation {
    pub file_path: &'static str,
    pub line_number: usize,
}

impl fmt::Display for StaticGlueCodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path, self.line_number)
    }
}
