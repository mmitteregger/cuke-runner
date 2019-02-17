use std::fmt::Debug;

use gherkin::cuke::Tag;

use super::TestStep;


pub trait TestCase: Debug + Send + Sync {
    fn get_test_steps(&self) -> Vec<TestStep>;

    fn get_name(&self) -> &str;

    fn get_scenario_designation(&self) -> String;

    fn get_uri(&self) -> &str;

    fn get_line(&self) -> u32;

    fn get_tags(&self) -> &[Tag];
}
