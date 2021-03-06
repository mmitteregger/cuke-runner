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
/// ```rust
/// # use cuke_runner::then;
/// # use cuke_runner::glue::scenario::{FromScenarioError, FromScenario, Scenario};
/// #
/// # pub struct Calc;
/// # impl Calc {
/// #     pub fn value(&self) -> f64 { 0f64 }
/// # }
/// #
/// # impl<'a> FromScenario<'a> for &'a Calc {
/// #     fn from_scenario(scenario: &'a Scenario) -> Result<&'a Calc, FromScenarioError> {
/// #         scenario.get::<Calc>()
/// #             .ok_or_else(|| FromScenarioError::new("Could not get calc from scenario"))
/// #     }
/// # }
/// #[then("the result is:")]
/// pub fn assert_doc_string_result(#[scenario] calc: &Calc, expected: f64) {
///     assert_eq!(calc.value(), expected);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DocString<'s> {
    cuke_string: &'s CukeString<'s>,
}

impl<'s> DocString<'s> {
    /// Returns the parsed value (arguments already replaced) from the doc string.
    pub fn value(&self) -> &str {
        &self.cuke_string.content
    }

    /// Returns the optional content type of the doc string.
    ///
    /// # Examples
    ///
    /// With the step:
    /// ```gherkin
    ///   Then the result is:
    ///   """
    ///   6
    ///   """
    /// ```
    /// The returned content type is `None`.
    ///
    /// But when the step is:
    /// ```gherkin
    ///   Then the result is:
    ///   """text
    ///   6
    ///   """
    /// ```
    /// The returned content type is `Some("text")`.
    #[deprecated(note = "superseded by `media_type`")]
    pub fn content_type(&self) -> Option<&str> {
        if self.cuke_string.media_type.is_empty() {
            None
        } else {
            Some(self.cuke_string.media_type.as_ref())
        }
    }

    /// Returns the media type of the doc string.
    ///
    /// # Examples
    ///
    /// With the step:
    /// ```gherkin
    ///   Then the result is:
    ///   """
    ///   6
    ///   """
    /// ```
    /// The returned media type is an empty string `""`.
    ///
    /// But when the step is:
    /// ```gherkin
    ///   Then the result is:
    ///   """text
    ///   6
    ///   """
    /// ```
    /// The returned media type is `"text"`.
    pub fn media_type(&self) -> &str {
        self.cuke_string.media_type.as_ref()
    }

    /// Returns the line of the doc string start delimiter.
    ///
    /// The line in the feature file where the doc string start delimiter `"""` is declared,
    /// which is also the place where the content type may be specified.
    ///
    /// # Examples
    ///
    /// With the line numbers of the step:
    /// ```text
    /// 20|  Then the result is:
    /// 21|  # DocStrings may be useful for rather small multiline text (e.g. json)
    /// 22|  """text
    /// 23|  6
    /// 24|  """
    /// ```
    /// The returned line is `22`.
    pub fn line(&self) -> u32 {
        self.cuke_string.location.line
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
