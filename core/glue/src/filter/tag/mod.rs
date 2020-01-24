mod expression;
#[doc(hidden)]
pub mod parser;

use gherkin::cuke::Tag;
use self::expression::Expression;

#[derive(Debug, Clone)]
pub struct TagPredicate<'p> {
    text: &'p str,
    expression: Expression,
}

impl<'p> TagPredicate<'p> {
    pub fn new(tag_expression: &'p str) -> Result<TagPredicate<'p>, String> {
        let tag_predicate = TagPredicate {
            text: tag_expression,
            expression: parser::parse(tag_expression)?,
        };
        Ok(tag_predicate)
    }

    pub fn apply(&self, tags: &[Tag]) -> bool {
        self.expression.evaluate(tags)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use gherkin::cuke::{Tag, Location};

    #[test]
    fn empty_tag_predicate_matches_with_any_tags() {
        let tags = create_tags(&["@FOO"]);
        let predicate = TagPredicate::new("").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn single_tag_predicate_does_not_match_with_no_tags() {
        let tags = create_tags(&[]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(!predicate.apply(&tags));
    }

    #[test]
    fn single_tag_predicate_matches_with_same_single_tag() {
        let tags = create_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn single_tag_predicate_matches_with_more_tags() {
        let tags = create_tags(&["@FOO", "@BAR"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn single_tag_predicate_does_not_match_with_different_single_tag() {
        let tags = create_tags(&["@BAR"]);
        let predicate = TagPredicate::new("@FOO").unwrap();

        assert!(!predicate.apply(&tags));
    }

    #[test]
    fn not_tag_predicate_matches_with_no_tags() {
        let tags = create_tags(&[]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn not_tag_predicate_does_not_match_with_same_single_tag() {
        let tags = create_tags(&["@FOO"]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(!predicate.apply(&tags));
    }

    #[test]
    fn not_tag_predicate_matches_with_different_single_tag() {
        let tags = create_tags(&["@BAR"]);
        let predicate = TagPredicate::new("not @FOO").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn and_tag_predicate_matches_with_all_tags() {
        let tags = create_tags(&["@FOO", "@BAR"]);
        let predicate = TagPredicate::new("@FOO and @BAR").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn and_tag_predicate_does_not_match_with_one_of_the_tags() {
        let tags = create_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO and @BAR").unwrap();

        assert!(!predicate.apply(&tags));
    }

    #[test]
    fn or_tag_predicate_matches_with_one_of_the_tags() {
        let tags = create_tags(&["@FOO"]);
        let predicate = TagPredicate::new("@FOO or @BAR").unwrap();

        assert!(predicate.apply(&tags));
    }

    #[test]
    fn or_tag_predicate_does_not_match_none_of_the_tags() {
        let tags = create_tags(&[]);
        let predicate = TagPredicate::new("@FOO or @BAR").unwrap();

        assert!(!predicate.apply(&tags));
    }

    fn create_tags<'a>(tags: &[&'a str]) -> Vec<Tag<'a>> {
        tags.iter()
            .map(|tag| Tag {
                location: Location {
                    line: 0,
                    column: 0,
                },
                name: tag,
                ast_node_id: "test",
            })
            .collect()
    }

}
