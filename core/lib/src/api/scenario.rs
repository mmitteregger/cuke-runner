use super::TestResultStatus;

/// BeforeScenario or AfterScenario hooks can receive a reference of this struct.
/// It allows writing text and embedding media into reports.
/// When declared in an AfterScenario block it also can inspect the test results.
pub trait Scenario {
    /// The **most severe** status of the Scenario's steps.
    fn get_status(&self) -> TestResultStatus;

    /// Embeds data into the report(s).
    ///
    /// Some reporters (such as the progress one) don't embed data, but others do (html and json).
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Embed a screenshot.
    /// // See your UI automation tool's docs for details about how to take a screenshot.
    /// scenario.embed(&png_bytes, "image/png".into_owned());
    /// ```
    fn embed(&self, data: &[u8], mime_type: String);

    /// Outputs some text into the report.
    fn write(&self, text: String);

    /// The name of the Scenario.
    fn get_name(&self) -> &String;

    /// The id of the Scenario.
    fn get_id(&self) -> &String;

    /// The uri of the feature file of the Scenario.
    fn get_uri(&self) -> &String;

    /// Return the line(s) in the feature file of the Scenario.
    ///
    /// Scenarios from Scenario Outlines return both the line of the example row
    /// and the line of the scenario outline.
    fn get_lines(&self) -> &Vec<u32>;
}
