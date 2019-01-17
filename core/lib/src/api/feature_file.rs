use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use gherkin::ast::Feature;

#[derive(Debug)]
pub struct FeatureFile {
    pub uri: String,
    pub feature: Feature,
}

impl PartialEq for FeatureFile {
    fn eq(&self, other: &Self) -> bool {
        self.uri.eq(&other.uri)
    }
}

impl Hash for FeatureFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uri.hash(state)
    }
}

impl Eq for FeatureFile {}
