use gherkin::cuke::Cuke;

use crate::filter::line::LinePredicate;
use crate::filter::name::NamePredicate;
use crate::filter::tag::TagPredicate;

#[doc(hidden)]
pub mod line;
#[doc(hidden)]
pub mod name;
#[doc(hidden)]
pub mod tag;

pub enum CukePredicate<'p> {
    Line(LinePredicate<'p>),
    Name(NamePredicate),
    Tag(TagPredicate<'p>),
}

impl<'p> CukePredicate<'p> {
    pub fn apply(&self, uri: &str, cuke: &Cuke<'_>) -> bool {
        match self {
            CukePredicate::Line(line_predicate) => {
                line_predicate.apply(uri, &cuke.locations)
            }
            CukePredicate::Name(name_predicate) => {
                name_predicate.apply(cuke.name.as_ref())
            }
            CukePredicate::Tag(tag_predicate) => {
                tag_predicate.apply(&cuke.tags)
            }
        }
    }
}
