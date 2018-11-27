//! Step keywords like "Given", "When" and "Then".

use std::fmt;
use std::str::FromStr;

use self::StepKeyword::*;

/// A step keyword like "Given", "When" and "Then".
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum StepKeyword {
    Given,
    When,
    Then,
    Star,
}

impl StepKeyword {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match *self {
            Given => "Given",
            When => "When",
            Then => "Then",
            Star => "*",
        }
    }
}

impl FromStr for StepKeyword {
    type Err = ();

    fn from_str(s: &str) -> Result<StepKeyword, ()> {
        match s.to_lowercase().as_ref() {
            "given" => Ok(Given),
            "when" => Ok(When),
            "then" => Ok(Then),
            "*" => Ok(Star),
            _ => panic!("bad step keyword: {}", s),
        }
    }
}

impl fmt::Display for StepKeyword {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
