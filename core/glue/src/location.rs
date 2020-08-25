use std::fmt;

#[derive(Debug, Clone)]
pub struct StaticGlueCodeLocation {
    pub file: &'static str,
    pub line: u32,
}

impl fmt::Display for StaticGlueCodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file, self.line)
    }
}
