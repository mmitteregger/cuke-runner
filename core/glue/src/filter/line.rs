use std::collections::HashMap;

use gherkin::cuke::Location;

pub struct LinePredicate<'p> {
    line_filters: HashMap<&'p str, Vec<u32>>,
}

impl<'p> LinePredicate<'p> {
    pub fn new(line_filters: HashMap<&'p str, Vec<u32>>) -> LinePredicate {
        LinePredicate {
            line_filters,
        }
    }

    pub fn apply(&self, uri: &str, locations: &[Location]) -> bool {
        match self.line_filters.get(uri) {
            Some(lines) => {
                for line in lines {
                    for location in locations {
                        if *line == location.line {
                            return true;
                        }
                    }
                }

                false
            },
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn matches_pickles_from_files_not_in_the_predicate_map() {
        // the argument "path/file.feature another_path/file.feature:8"
        // results in only line predicates only for another_path/file.feature,
        // but all pickles from path/file.feature shall also be executed.
        let predicate = line_predicate("another_path/file.feature", vec![8]);

        assert!(predicate.apply("path/file.feature", &[location(4)]));
    }

    #[test]
    pub fn matches_pickles_for_any_line_in_predicate() {
        let predicate = line_predicate("path/file.feature", vec![4, 8]);

        assert!(predicate.apply("path/file.feature", &[location(8)]));
    }

    #[test]
    pub fn matches_pickles_on_any_location_of_the_pickle() {
        let predicate = line_predicate("path/file.feature", vec![8]);

        assert!(predicate.apply("path/file.feature", &[location(4), location(8)]));
    }

    #[test]
    pub fn does_not_matches_pickles_not_on_any_line_of_the_predicate() {
        let predicate = line_predicate("path/file.feature", vec![10]);

        assert!(!predicate.apply("path/file.feature", &[location(4), location(8)]));
    }

    fn location(line: u32) -> Location {
        Location {
            line,
            column: 0,
        }
    }

    fn line_predicate(uri: &str, lines: Vec<u32>) -> LinePredicate {
        let mut line_filters = HashMap::with_capacity(1);
        line_filters.insert(uri, lines);
        LinePredicate::new(line_filters)
    }
}
