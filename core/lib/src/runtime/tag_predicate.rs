use gherkin::pickle::PickleTag;

#[derive(Debug, Clone)]
struct Expression;

// TODO: support tag predicates
#[derive(Debug, Clone)]
pub struct TagPredicate {
    expressions: Vec<Expression>,
}

impl TagPredicate {
    pub fn new(_tag_expressions: Vec<String>) -> TagPredicate {
        TagPredicate {
            expressions: Vec::new(),
        }
    }

    pub fn test(&self, pickle_tags: &[PickleTag]) -> bool {
        // TODO: apply tag expression
        true
    }
}


//public class TagPredicate implements PicklePredicate {
//    private final List<Expression> expressions = new ArrayList<Expression>();
//    private final List<TagExpressionOld> oldStyleExpressions = new ArrayList<TagExpressionOld>();
//
//    public TagPredicate(List<String> tagExpressions) {
//        if (tagExpressions == null) {
//            return;
//        }
//        TagExpressionParser parser = new TagExpressionParser();
//        for (String tagExpression : tagExpressions) {
//            if (TagExpressionOld.isOldTagExpression(tagExpression)) {
//                oldStyleExpressions.add(new TagExpressionOld(asList(tagExpression)));
//            } else {
//                expressions.add(parser.parse(tagExpression));
//            }
//        }
//    }
//
//    @Override
//    public boolean apply(PickleEvent pickleEvent) {
//        return apply(pickleEvent.pickle.getTags());
//    }
//
//    public boolean apply(Collection<PickleTag> pickleTags) {
//        for (TagExpressionOld oldStyleExpression : oldStyleExpressions) {
//            if (!oldStyleExpression.evaluate(pickleTags)) {
//                return false;
//            }
//        }
//        List<String> tags = new ArrayList<String>();
//        for (PickleTag pickleTag : pickleTags) {
//            tags.add(pickleTag.getName());
//        }
//        for (Expression expression : expressions) {
//            if (!expression.evaluate(tags)) {
//                return false;
//            }
//        }
//        return true;
//    }
//
//}
