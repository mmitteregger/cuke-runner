use cuke_runner::given;

cuke_runner::generate_glue!();

#[given(".*")]
pub fn all() {
    // Try to uncomment the next line to see the progress bar in action
    // std::thread::sleep(std::time::Duration::from_millis(50));
}
