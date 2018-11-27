/// Represents an argument for a step definition.
///
/// The step definition `I have (\d+) cukes in my belly`
/// when matched with `I have 7 cukes in my belly` will produce one argument with value `"4"`,
/// starting at `7` and ending at `8`.
pub trait Argument: Sized {
    fn get_value(&self) -> String;

    fn get_start(&self) -> u32;

    fn get_end(&self) -> u32;
}