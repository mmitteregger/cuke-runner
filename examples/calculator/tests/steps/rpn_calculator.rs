//use cuke_runner::State;
use cuke_runner::glue::{Scenario, FromScenario, FromScenarioError};

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
        scenario.get_by_type::<Calc>()
            .ok_or_else(|| FromScenarioError::new("Could not get calc from scenario"))
    }
}

// #[before_scenario]
pub fn init() {
//    State::init(|| RpnCalculator::new());
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

// Another option:
//#[given("a calculator I just turned on")]
//pub fn reset_calculator_2() {
//    let mut calc = State::<RpnCalculator>::get();
//    calc.reset();
//}

#[when("I add (\\d+) and (\\d+)")]
pub fn add(arg1: &str, arg2: &str, calc: &mut Calc) {
    calc.push(arg1);
    calc.push(arg2);
    calc.push("+");
}

#[given("I press (.+)")]
pub fn press(what: String, calc: &mut Calc) {
    calc.push(what)
}

#[then("the result is (.*)")]
pub fn assert_result(expected: f64, calc: &mut Calc) {
    assert_eq!(calc.value(), expected);
}

//
//After((Scenario scenario) -> {
//    // result.write("HELLLLOO");
//});
//
//
//Given("the previous entries:", (DataTable dataTable) -> {
//    List<Entry> entries = dataTable.asList(Entry.class);
//    for (Entry entry : entries) {
//        calc.push(entry.first);
//        calc.push(entry.second);
//        calc.push(entry.operation);
//    }
//});
//
//static final class Entry {
//    private final Integer first;
//    private final Integer second;
//    private final String operation;
//
//    Entry(Integer first, Integer second, String operation) {
//        this.first = first;
//        this.second = second;
//        this.operation = operation;
//    }
//}
