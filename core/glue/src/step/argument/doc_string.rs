use gherkin::pickle::PickleString;

#[derive(Debug, Clone)]
pub struct DocString {
    pub(crate) value: String,
}

impl DocString {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<&PickleString> for DocString {
    fn from(pickle_string: &PickleString) -> Self {
        DocString {
            value: pickle_string.content.clone(),
        }
    }
}
