use gherkin::pickle::PickleString;

/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
#[derive(Debug, Clone)]
pub struct DocString<'s> {
    pickle_string: &'s PickleString,
}

impl<'s> DocString<'s> {
    pub fn value(&self) -> &str {
        &self.pickle_string.content
    }
}

impl<'s> From<&'s PickleString> for DocString<'s> {
    fn from(pickle_string: &'s PickleString) -> Self {
        DocString {
            pickle_string,
        }
    }
}
