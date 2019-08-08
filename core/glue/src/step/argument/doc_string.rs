use gherkin::cuke::String as CukeString;

/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
#[derive(Debug, Clone)]
pub struct DocString<'s> {
    cuke_string: &'s CukeString<'s>,
}

impl<'s> DocString<'s> {
    pub fn value(&self) -> &str {
        &self.cuke_string.content
    }
}

#[doc(hidden)]
impl<'s> From<&'s CukeString<'s>> for DocString<'s> {
    fn from(cuke_string: &'s CukeString<'s>) -> Self {
        DocString {
            cuke_string,
        }
    }
}
