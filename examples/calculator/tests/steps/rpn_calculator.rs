use cuke_runner::glue::scenario::{Scenario, FromScenario, FromScenarioError};
use cuke_runner::glue::step::argument::{DataTable, FromDataTableRow, FromStepArgumentResult};

use calculator::RpnCalculator;

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

impl<'a> FromScenario<'a> for &'a mut Calc {
    fn from_scenario(scenario: &'a mut Scenario) -> Result<&'a mut Calc, FromScenarioError> {
        scenario.get_user_data::<Calc>()
            .ok_or_else(|| FromScenarioError::new("Could not get calc from scenario"))
    }
}

#[before_scenario]
pub fn init(scenario: &mut Scenario) {
    scenario.set_user_data(Calc(RpnCalculator::new()));
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
pub fn reset_calculator(calc: &mut Calc) {
    calc.reset();
}

#[when("I add (\\d+) and (\\d+)")]
pub fn add(calc: &mut Calc, arg1: &str, arg2: &str) {
    calc.push(arg1);
    calc.push(arg2);
    calc.push("+");
}

#[given("I press (.+)")]
pub fn press(calc: &mut Calc, what: &str) {
    calc.push(what)
}

#[then("the result is (.*)")]
pub fn assert_result(calc: &mut Calc, expected: f64) {
    assert_eq!(calc.value(), expected);
}

#[given("the previous entries:")]
pub fn previous_entries(calc: &mut Calc, data_table: &DataTable) {
    let entries: Vec<Entry> = data_table.to_vec().unwrap();

    for entry in entries {
        calc.push(entry.first);
        calc.push(entry.second);
        calc.push(entry.operation);
    }
}

struct Entry<'r> {
    first: &'r str,
    second: &'r str,
    operation: &'r str,
}

impl<'r> FromDataTableRow<'r> for Entry<'r> {
    fn from_data_table_row<S: AsRef<str>>(row: &'r [S]) -> FromStepArgumentResult<Self> {
        Ok(Entry {
            first: row[0].as_ref(),
            second: row[1].as_ref(),
            operation: row[2].as_ref(),
        })
    }
}
