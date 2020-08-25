/*!
Core event listeners and reporter plugins for cuke-runner.

The listeners are gated behind cargo features and are only available when enabled.\
This way only the required listeners and their dependencies need to be compiled.\
Look at the documentation of the individual listeners for the required feature names.
*/

#![warn(rust_2018_idioms)]

#[cfg(feature = "pretty_print")]
mod pretty_print;
#[cfg(feature = "pretty_print")]
pub use pretty_print::PrettyPrintListener;

#[cfg(feature = "progress_bar")]
mod progress_bar;
#[cfg(feature = "progress_bar")]
pub use progress_bar::{ProgressBarListener, ProgressStyle};

#[cfg(feature = "json_report")]
mod json_report;
#[cfg(feature = "json_report")]
pub use json_report::JsonReportListener;
