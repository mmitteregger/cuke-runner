use regex::Regex;

pub struct NamePredicate {
    patterns: Vec<Regex>,
}

impl NamePredicate {
    pub fn new(patterns: &[&str]) -> NamePredicate {
        let name_patterns = patterns.iter()
            .map(|pattern| {
                match Regex::new(pattern) {
                    Ok(regex) => regex,
                    Err(error) => panic!("invalid name pattern: {}", error),
                }
            })
            .collect();

        NamePredicate {
            patterns: name_patterns,
        }
    }

    pub fn apply(&self, cuke_name: &str) -> bool {
        for pattern in &self.patterns {
            if pattern.is_match(cuke_name) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn anchored_name_pattern_matches_exact_name() {
        let predicate = NamePredicate::new(&["^a pickle name$"]);

        assert!(predicate.apply("a pickle name"));
    }

    #[test]
    pub fn anchored_name_pattern_does_not_match_part_of_name() {
        let predicate = NamePredicate::new(&["^a pickle name$"]);

        assert!(!predicate.apply("a pickle name with suffix"));
    }

    #[test]
    pub fn non_anchored_name_pattern_matches_part_of_name() {
        let predicate = NamePredicate::new(&["a pickle name"]);

        assert!(predicate.apply("a pickle name with suffix"));
    }

    #[test]
    pub fn wildcard_name_pattern_matches_part_of_name() {
        let predicate = NamePredicate::new(&["a .* name"]);

        assert!(predicate.apply("a pickleEvent name"));
    }
}
