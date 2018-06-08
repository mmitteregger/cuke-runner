use std::collections::HashMap;

use super::HookDefinition;
use super::StepDefinition;

#[derive(Default)]
pub struct Glue {
    step_definitions_by_pattern: HashMap<String, StepDefinition>,
    before_scenario_hooks: Vec<HookDefinition>,
    before_step_hooks: Vec<HookDefinition>,
    after_step_hooks: Vec<HookDefinition>,
    after_scenario_hooks: Vec<HookDefinition>,
}

impl Glue {
    pub fn new() -> Glue {
        Glue::default()
    }

    pub fn get_before_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_scenario_hooks
    }

    pub fn get_after_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_scenario_hooks
    }

    pub fn get_before_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_step_hooks
    }

    pub fn get_after_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_step_hooks
    }

}

//public class RuntimeGlue implements Glue {
//    final Map<String, StepDefinition> stepDefinitionsByPattern = new TreeMap<String, StepDefinition>();
//    final List<HookDefinition> beforeHooks = new ArrayList<HookDefinition>();
//    final List<HookDefinition> beforeStepHooks = new ArrayList<HookDefinition>();
//    final List<HookDefinition> afterHooks = new ArrayList<HookDefinition>();
//    final List<HookDefinition> afterStepHooks = new ArrayList<HookDefinition>();
//    final Map<String, CacheEntry> matchedStepDefinitionsCache = new HashMap<String, CacheEntry>();
//
//    @Override
//    public void addStepDefinition(StepDefinition stepDefinition) {
//        StepDefinition previous = stepDefinitionsByPattern.get(stepDefinition.getPattern());
//        if (previous != null) {
//            throw new DuplicateStepDefinitionException(previous, stepDefinition);
//        }
//        stepDefinitionsByPattern.put(stepDefinition.getPattern(), stepDefinition);
//    }
//
//    @Override
//    public void addBeforeHook(HookDefinition hookDefinition) {
//        beforeHooks.add(hookDefinition);
//        Collections.sort(beforeHooks, new HookComparator(true));
//    }
//
//    @Override
//    public void addBeforeStepHook(HookDefinition hookDefinition) {
//        beforeStepHooks.add(hookDefinition);
//        Collections.sort(beforeStepHooks, new HookComparator(true));
//    }
//    @Override
//    public void addAfterHook(HookDefinition hookDefinition) {
//        afterHooks.add(hookDefinition);
//        Collections.sort(afterHooks, new HookComparator(false));
//    }
//
//    @Override
//    public void addAfterStepHook(HookDefinition hookDefinition) {
//        afterStepHooks.add(hookDefinition);
//        Collections.sort(afterStepHooks, new HookComparator(false));
//    }
//
//    @Override
//    public List<HookDefinition> getBeforeHooks() {
//        return beforeHooks;
//    }
//
//    @Override
//    public List<HookDefinition> getBeforeStepHooks() {
//        return beforeStepHooks;
//    }
//
//    @Override
//    public List<HookDefinition> getAfterHooks() {
//        return afterHooks;
//    }
//
//    @Override
//    public List<HookDefinition> getAfterStepHooks() {
//        return afterStepHooks;
//    }
//
//    @Override
//    public PickleStepDefinitionMatch stepDefinitionMatch(String featurePath, PickleStep step) {
//        String stepText = step.getText();
//
//        CacheEntry cacheEntry = matchedStepDefinitionsCache.get(stepText);
//        if (cacheEntry != null) {
//            return new PickleStepDefinitionMatch(Collections.<Argument>emptyList(), cacheEntry.stepDefinition, featurePath, step);
//        }
//
//        List<PickleStepDefinitionMatch> matches = stepDefinitionMatches(featurePath, step);
//        if (matches.isEmpty()) {
//            return null;
//        }
//        if (matches.size() > 1) {
//            throw new AmbiguousStepDefinitionsException(step, matches);
//        }
//
//        PickleStepDefinitionMatch match = matches.get(0);
//
//        // We can only cache step definitions without arguments.
//        // DocString and TableArguments are not included in the stepText used as the cache key.
//        if(match.getArguments().isEmpty()) {
//            matchedStepDefinitionsCache.put(stepText, new CacheEntry(match.getStepDefinition()));
//        }
//
//        return match;
//    }
//
//    private List<PickleStepDefinitionMatch> stepDefinitionMatches(String featurePath, PickleStep step) {
//        List<PickleStepDefinitionMatch> result = new ArrayList<PickleStepDefinitionMatch>();
//        for (StepDefinition stepDefinition : stepDefinitionsByPattern.values()) {
//            List<Argument> arguments = stepDefinition.matchedArguments(step);
//            if (arguments != null) {
//                result.add(new PickleStepDefinitionMatch(arguments, stepDefinition, featurePath, step));
//            }
//        }
//        return result;
//    }
//
//    @Override
//    public void reportStepDefinitions(StepDefinitionReporter stepDefinitionReporter) {
//        for (StepDefinition stepDefinition : stepDefinitionsByPattern.values()) {
//            stepDefinitionReporter.stepDefinition(stepDefinition);
//        }
//    }
//
//    @Override
//    public void removeScenarioScopedGlue() {
//        removeScenarioScopedHooks(beforeHooks);
//        removeScenarioScopedHooks(beforeStepHooks);
//        removeScenarioScopedHooks(afterHooks);
//        removeScenarioScopedHooks(afterStepHooks);
//        removeScenarioScopedStepdefs();
//    }
//
//    private void removeScenarioScopedHooks(List<HookDefinition> beforeHooks1) {
//        Iterator<HookDefinition> hookIterator = beforeHooks1.iterator();
//        while (hookIterator.hasNext()) {
//            HookDefinition hook = hookIterator.next();
//            if (hook.isScenarioScoped()) {
//                hookIterator.remove();
//            }
//        }
//    }
//
//    private void removeScenarioScopedStepdefs() {
//        Iterator<Map.Entry<String, StepDefinition>> stepdefs = stepDefinitionsByPattern.entrySet().iterator();
//        while (stepdefs.hasNext()) {
//            StepDefinition stepDefinition = stepdefs.next().getValue();
//            if (stepDefinition.isScenarioScoped()) {
//                stepdefs.remove();
//            }
//        }
//
//        Iterator<Map.Entry<String, CacheEntry>> cachedStepDefs = matchedStepDefinitionsCache.entrySet().iterator();
//        while(cachedStepDefs.hasNext()){
//            StepDefinition stepDefinition = cachedStepDefs.next().getValue().stepDefinition;
//            if(stepDefinition.isScenarioScoped()){
//                cachedStepDefs.remove();
//            }
//        }
//    }
//
//    static final class CacheEntry {
//
//        StepDefinition stepDefinition;
//
//        private CacheEntry(StepDefinition stepDefinition) {
//            this.stepDefinition = stepDefinition;
//        }
//    }
//}
