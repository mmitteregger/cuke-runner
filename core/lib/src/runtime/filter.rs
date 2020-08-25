use gherkin::cuke::Cuke;

use crate::Config;
use crate::glue::filter::CukePredicate;
use crate::glue::filter::tag::TagPredicate;

pub struct Filters<'f> {
    filters: Vec<CukePredicate<'f>>,
}

impl<'f> Filters<'f> {
    pub fn apply(&self, uri: &str, cuke: &Cuke<'_>) -> bool {
        for filter in &self.filters {
            if !filter.apply(uri, cuke) {
                return false;
            }
        }

        true
    }
}

impl<'f> From<&'f Config<'f>> for Filters<'f> {
    fn from(config: &'f Config<'f>) -> Self {
        let mut filters = Vec::with_capacity(config.tags.len());

        for tag in config.tags {
            let tag_predicate = TagPredicate::new(tag)
                .expect("invalid tag filter");
            filters.push(CukePredicate::Tag(tag_predicate));
        }

        Filters {
            filters,
        }
    }
}
