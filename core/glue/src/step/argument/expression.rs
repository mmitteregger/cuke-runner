/// A step argument expression is the simplest form of a step argument.
/// It is parsed out of the step text by the supplied regular expression.
///
/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
///
/// # Examples
///
/// With the scenario:
/// ```gherkin
/// Scenario: Addition
///   When I add 4 and 5
///   Then the result is 9
/// ```
/// and the glue code:
/// ```rust
/// # #![feature(custom_attribute)]
/// #
/// # pub struct Calc;
/// # impl Calc {
/// #     pub fn push(&mut self, s: &str) {}
/// #     pub fn value(&self) -> f64 { 0f64 }
/// # }
///
/// #[when("I add (\\d+) and (\\d+)")]
/// pub fn add(calc: &mut Calc, arg1: &str, arg2: &str) {
///     calc.push(arg1);
///     calc.push(arg2);
///     calc.push("+");
/// }
///
/// #[then("the result is (.*)")]
/// pub fn assert_result(calc: &mut Calc, expected: f64) {
///     assert_eq!(calc.value(), expected);
/// }
/// ```
/// The following expressions will be parsed:
/// * `arg1` with value `4`
/// * `arg2` with value `5`
/// * `expected` with value `9`
#[derive(Debug, Clone)]
pub struct Expression<'s> {
    value: &'s str,
    start: usize,
    end: usize,
}

impl<'s> Expression<'s> {
    /// Returns the parsed value from the step text.
    pub fn value(&'s self) -> &'s str {
        &self.value
    }
    /// Returns the start index of the matched value from the step text.
    pub fn start(&self) -> usize {
        self.start
    }
    /// Returns the end index of the matched value from the step text.
    pub fn end(&self) -> usize {
        self.end
    }
}

#[doc(hidden)]
impl<'s, 't: 's> From<regex::Match<'t>> for Expression<'s> {
    fn from(mat: regex::Match<'t>) -> Self {
        Expression {
            value: mat.as_str(),
            start: mat.start(),
            end: mat.end(),
        }
    }
}
