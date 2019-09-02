use calculator::RpnCalculator;
use cuke_runner::glue::scenario::{FromScenarioError, FromScenarioMut, Scenario};
use cuke_runner::glue::step::argument::{BodyRowRef, DataTable, FromDataTableBodyRow};

#[derive(Debug)]
pub struct Calc(RpnCalculator);

impl ::std::ops::Deref for Calc {
    type Target = RpnCalculator;

    fn deref(&self) -> &RpnCalculator {
        &self.0
    }
}

impl ::std::ops::DerefMut for Calc {
    fn deref_mut(&mut self) -> &mut RpnCalculator {
        &mut self.0
    }
}

impl<'a> FromScenarioMut<'a> for &'a mut Calc {
    fn from_scenario_mut(scenario: &'a mut Scenario) -> Result<&'a mut Calc, FromScenarioError> {
        scenario.get_mut::<Calc>()
            .ok_or_else(|| FromScenarioError::new("Could not get calc from scenario"))
    }
}

#[before_scenario]
pub fn init(scenario: &mut Scenario) {
    scenario.set(Calc(RpnCalculator::new()));
}

// // Other hooks and attributes that should be supported:
// #[before_scenario(order, tags)]
// #[after_scenario(order, tags)]
// #[before_step(order, tags)]
// #[after_step(order, tags)]
// // hook that should be executed before the tests are run (one-only global setup hook)
// #[before_all(order)]
// // reverse of #before_all
// #[after_all(order)]
// // not sure about this one yet...
// // #[after_configuration] taking configuration as function argument

#[given("a calculator I just turned on")]
pub fn reset_calculator(#[scenario] calc: &mut Calc) {
    calc.reset();
}

#[when("I add (\\d+) and (\\d+)")]
pub fn add(#[scenario] calc: &mut Calc, arg1: &str, arg2: &str) {
    calc.push(arg1);
    calc.push(arg2);
    calc.push("+");
}

#[given("I press (.+)")]
pub fn press(#[scenario] calc: &mut Calc, what: &str) {
    calc.push(what)
}

#[then("the result is (.*)")]
pub fn assert_result(#[scenario] calc: &mut Calc, expected: f64) {
    assert_eq!(calc.value(), expected);
}

#[then("the result is:")]
pub fn assert_doc_string_result(#[scenario] calc: &mut Calc, expected: f64) {
    assert_eq!(calc.value(), expected);
}

#[given("the previous entries:")]
pub fn previous_entries(#[scenario] calc: &mut Calc, data_table: &DataTable) {
    for entry in data_table.body_rows::<Entry>() {
        calc.push(entry.first);
        calc.push(entry.second);
        calc.push(entry.operation);
    }
}

struct Entry<'dt> {
    first: &'dt str,
    second: &'dt str,
    operation: &'dt str,
}

impl<'dt> FromDataTableBodyRow<'dt> for Entry<'dt> {
    fn from(body_row: BodyRowRef<'_, 'dt>) -> Self {
        Entry {
            first: body_row["first"],
            second: body_row["second"],
            operation: body_row["operation"],
        }
    }
}
