use cuke_runner::State;

use calculator::RpnCalculator;

#[given("a calculator I just turned on")]
pub fn reset_calculator(mut calc: State<RpnCalculator>) {
    calc.reset();
}

#[when("I add (\\d+) and (\\d+)")]
pub fn add(mut calc: State<RpnCalculator>, arg1: f64, arg2: f64) {
    calc.push(arg1);
    calc.push(arg2);
    calc.push("+");
}

#[given("I press (.+)")]
pub fn press(mut calc: State<RpnCalculator>, what: String) {
    calc.push(what)
}

#[then("the result is (.*)")]
pub fn assert_result(calc: State<RpnCalculator>, expected: f64) {
    assert_equals!(calc.value(), expected);
}

//Before(new String[]{"not @foo"}, (Scenario scenario) -> {
//    scenario.write("Runs before scenarios *not* tagged with @foo");
//});
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
