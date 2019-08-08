use gherkin::cuke::String as CukeString;

/// A possibly multiline string that is attached to a step text.
///
/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
///
/// # Examples
///
/// With the scenario:
/// ```gherkin
/// Scenario: DocString Addition
///   When I add 4 and 2
///   Then the result is:
///   # DocStrings may be useful for rather small multiline text (e.g. json)
///   """text
///   6
///   """
/// ```
/// and the glue code:
/// ```rust,no-run
/// # #![feature(custom_attribute)]
/// #
/// # pub struct Calc;
/// # impl Calc {
/// #     pub fn value(&self) -> f64 { 0f64 }
/// # }
/// #[then("the result is:")]
/// pub fn assert_doc_string_result(calc: &Calc, expected: f64) {
///     assert_eq!(calc.value(), expected);
/// }
/// ```
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
