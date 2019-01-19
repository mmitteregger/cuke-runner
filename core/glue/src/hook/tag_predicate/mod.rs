mod expression;
#[doc(hidden)]
pub mod parser;

use gherkin::pickle::PickleTag;
use self::expression::Expression;

#[derive(Debug, Clone)]
pub struct TagPredicate {
    text: &'static str,
    expression: Expression,
}

impl TagPredicate {
    pub fn new(tag_expression: &'static str) -> Result<TagPredicate, String> {
        let tag_predicate = TagPredicate {
            text: tag_expression,
            expression: parser::parse(tag_expression)?,
        };
        Ok(tag_predicate)
    }

    pub fn test(&self, pickle_tags: &[PickleTag]) -> bool {
        self.expression.evaluate(pickle_tags)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use gherkin::pickle::{PickleTag, PickleLocation};

    #[test]
    fn empty_tag_predicate_matches_pickle_with_any_tags() {
        let tags = create_pickle_tags(&["@FOO"]);
        let predicate = TagPredicate::new("").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn single_tag_predicate_does_not_match_pickle_with_no_tags() {
        let tags = create_pickle_tags(&[]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(!predicate.test(&tags));
    }

    #[test]
    fn single_tag_predicate_matches_pickle_with_same_single_tag() {
        let tags = create_pickle_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn single_tag_predicate_matches_pickle_with_more_tags() {
        let tags = create_pickle_tags(&["@FOO", "@BAR"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn single_tag_predicate_does_not_match_pickle_with_different_single_tag() {
        let tags = create_pickle_tags(&["@BAR"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(!predicate.test(&tags));
    }

    #[test]
    fn not_tag_predicate_matches_pickle_with_no_tags() {
        let tags = create_pickle_tags(&[]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn not_tag_predicate_does_not_match_pickle_with_same_single_tag() {
        let tags = create_pickle_tags(&["@FOO"]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(!predicate.test(&tags));
    }

    #[test]
    fn not_tag_predicate_matches_pickle_with_different_single_tag() {
        let tags = create_pickle_tags(&["@BAR"]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn and_tag_predicate_matches_pickle_with_all_tags() {
        let tags = create_pickle_tags(&["@FOO", "@BAR"]);
        let predicate = TagPredicate::new("@FOO and @BAR").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn and_tag_predicate_does_not_match_pickle_with_one_of_the_tags() {
        let tags = create_pickle_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO and @BAR").unwrap();

        assert!(!predicate.test(&tags));
    }

    #[test]
    fn or_tag_predicate_matches_pickle_with_one_of_the_tags() {
        let tags = create_pickle_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO or @BAR").unwrap();

        assert!(predicate.test(&tags));
    }

    #[test]
    fn or_tag_predicate_does_not_match_pickle_none_of_the_tags() {
        let tags = create_pickle_tags(&[]);
        let predicate = TagPredicate::new("@FOO or @BAR").unwrap();

        assert!(!predicate.test(&tags));
    }

    fn create_pickle_tags(tags: &[&str]) -> Vec<PickleTag> {
        tags.iter()
            .map(|tag| PickleTag {
                location: PickleLocation {
                    line: 0,
                    column: 0,
                },
                name: tag.to_string(),
            })
            .collect()
    }

}
