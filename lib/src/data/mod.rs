/*!
User supplied step definition data for.

This module contains step input data and conversions.
*/

mod step_keyword;
mod from_step_data;

pub use self::step_keyword::StepKeyword;
pub use self::from_step_data::FromStepData;
pub use self::from_step_data::Result as FromStepDataResult;
pub use self::from_step_data::Error as FromStepDataError;

pub struct StepData {

}
