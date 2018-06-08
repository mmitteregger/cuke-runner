use std::fmt::Debug;

use gherkin::pickle::PickleTag;

use super::TestStep;


pub trait TestCase: Debug + Send + Sync {
    fn get_test_steps(&self) -> &Vec<Box<TestStep>>;

    fn get_name(&self) -> &String;

    fn get_scenario_designation(&self) -> &String;

    fn get_uri(&self) -> &String;

    fn get_line(&self) -> usize;

    fn get_tags(&self) -> &Vec<PickleTag>;
}
