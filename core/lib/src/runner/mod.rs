pub use self::event_bus::EventBus;
pub use self::test_step::{HookTestStep, PickleStepTestStep};

mod event_bus;
mod test_step;

use gherkin::event::PickleEvent;
use gherkin::pickle::PickleTag;

use api::{HookType, FeatureFile};
use runtime::{Glue, HookDefinition};
use runtime::{self, TestCase, StepDefinitionMatch, HookDefinitionMatch};

pub struct Runner {
    glue: Glue,
    dry_run: bool,
}

impl Runner {
    pub fn new(glue: Glue, dry_run: bool) -> Runner {
        Runner {
            glue,
            dry_run,
        }
    }

    pub fn run_pickle(&self, feature: &FeatureFile, pickle_event: PickleEvent, event_bus: &EventBus) {
        let test_case = self.create_test_case_for_pickle_event(feature, &pickle_event);
        runtime::test_case::run(test_case, event_bus);
    }

    fn create_test_case_for_pickle_event<'c, 's: 'c>(&'s self,
        feature_file: &'c FeatureFile, pickle_event: &'c PickleEvent) -> TestCase<'c>
    {
        let (
            before_hooks,
            after_hooks,
            test_steps,
        ) = if pickle_event.pickle.steps.is_empty() {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        } else {
            let tags = &pickle_event.pickle.tags;
            (
                self.create_before_scenario_hooks(tags),
                self.create_after_scenario_hooks(tags),
                self.create_test_steps(pickle_event),
            )
        };

        TestCase {
            feature_file,
            pickle_event,
            test_steps,
            before_hooks,
            after_hooks,
            dry_run: self.dry_run,
        }
    }

    fn create_before_scenario_hooks(&self, tags: &[PickleTag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_before_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::BeforeScenario)
    }

    fn create_after_scenario_hooks(&self, tags: &[PickleTag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_after_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::AfterScenario)
    }

    fn create_hooks(&self, tags: &[PickleTag], hook_definitions: &[HookDefinition],
        hook_type: HookType) -> Vec<HookTestStep>
    {
        let mut hooks = Vec::with_capacity(hook_definitions.len());

        for hook_definition in hook_definitions {
            if hook_definition.matches(tags) {
                let test_step = HookTestStep {
                    definition_match: StepDefinitionMatch::Hook(HookDefinitionMatch {
                        hook_definition: hook_definition.clone(),
                        arguments: Vec::new(),
                    }),
                    hook_type,
                };
                hooks.push(test_step);
            }
        }

        hooks
    }

    fn create_test_steps<'e, 's: 'e>(&'s self, pickle_event: &'e PickleEvent) -> Vec<PickleStepTestStep<'e>> {
        let mut test_steps = Vec::new();

        let feature_path = &pickle_event.uri;
        let tags = &pickle_event.pickle.tags;

        for step in &pickle_event.pickle.steps {
            let step_definition_match = self.glue.step_definition_match(feature_path, step);
            let before_step_hook_steps = self.get_before_step_hooks(tags);
            let after_step_hook_steps = self.get_after_step_hooks(tags);

            test_steps.push(PickleStepTestStep {
                uri: feature_path.clone(),
                before_step_hook_steps,
                after_step_hook_steps,
                step_definition_match,
            });
        }

        test_steps
    }

    fn get_before_step_hooks(&self, tags: &[PickleTag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_before_step_hooks();
        self.create_hooks(tags, hook_definitions, HookType::BeforeStep)
    }

    fn get_after_step_hooks(&self, tags: &[PickleTag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_after_step_hooks();
        self.create_hooks(tags, hook_definitions, HookType::AfterStep)
    }
}

//public class Runner {
//
//    public Runner(Glue glue, EventBus bus, Collection<? extends Backend> backends, RuntimeOptions runtimeOptions) {
//        this.glue = glue;
//        this.bus = bus;
//        this.runtimeOptions = runtimeOptions;
//        this.backends = backends;
//        for (Backend backend : backends) {
//            backend.loadGlue(glue, runtimeOptions.getGlue());
//        }
//    }
//
//    public void runPickle(PickleEvent pickle) {
//        buildBackendWorlds(); // Java8 step definitions will be added to the glue here
//        TestCase testCase = createTestCaseForPickle(pickle);
//        testCase.run(bus);
//        disposeBackendWorlds();
//    }
//
//    public Glue getGlue() {
//        return glue;
//    }
//
//
//    public void reportStepDefinitions(StepDefinitionReporter stepDefinitionReporter) {
//        glue.reportStepDefinitions(stepDefinitionReporter);
//    }
//
//    private TestCase createTestCaseForPickle(PickleEvent pickleEvent) {
//        List<PickleStepTestStep> testSteps = new ArrayList<PickleStepTestStep>();
//        List<HookTestStep> beforeHooks = new ArrayList<HookTestStep>();
//        List<HookTestStep> afterHooks = new ArrayList<HookTestStep>();
//        if (!pickleEvent.pickle.getSteps().isEmpty()) {
//            add_test_steps_for_before_hooks(beforeHooks, pickleEvent.pickle.getTags());
//            addTestStepsForPickleSteps(testSteps, pickleEvent);
//            add_test_steps_for_after_hooks(afterHooks, pickleEvent.pickle.getTags());
//        }
//        return new TestCase(testSteps, beforeHooks, afterHooks, pickleEvent, runtimeOptions.isDryRun());
//    }
//
//    private void addTestStepsForPickleSteps(List<PickleStepTestStep> testSteps, PickleEvent pickleEvent) {
//        for (PickleStep step : pickleEvent.pickle.getSteps()) {
//            PickleStepDefinitionMatch match;
//            try {
//                match = glue.stepDefinitionMatch(pickleEvent.uri, step);
//                if (match == null) {
//                    List<String> snippets = new ArrayList<String>();
//                    for (Backend backend : backends) {
//                        String snippet = backend.getSnippet(step, "**KEYWORD**", runtimeOptions.getSnippetType().getFunctionNameGenerator());
//                        if (snippet != null) {
//                            snippets.add(snippet);
//                        }
//                    }
//                    if (!snippets.isEmpty()) {
//                        bus.send(new SnippetsSuggestedEvent(bus.getTime(), pickleEvent.uri, step.getLocations(), snippets));
//                    }
//                    match = new UndefinedPickleStepDefinitionMatch(step);
//                }
//            } catch (AmbiguousStepDefinitionsException e) {
//                match = new AmbiguousPickleStepDefinitionsMatch(pickleEvent.uri, step, e);
//            } catch (Throwable t) {
//                match = new FailedPickleStepInstantiationMatch(pickleEvent.uri, step, t);
//            }
//
//
//            List<HookTestStep> afterStepHookSteps = getAfterStepHooks(pickleEvent.pickle.getTags());
//            List<HookTestStep> beforeStepHookSteps = getBeforeStepHooks(pickleEvent.pickle.getTags());
//            testSteps.add(new PickleStepTestStep(pickleEvent.uri, step, beforeStepHookSteps, afterStepHookSteps, match));
//        }
//    }
//
//
//    private List<HookTestStep> getAfterStepHooks(List<PickleTag> tags) {
//        List<HookTestStep> hookSteps = new ArrayList<HookTestStep>();
//        add_test_steps_for_hooks(hookSteps, tags, glue.getAfterStepHooks(), HookType.AfterStep);
//        return hookSteps;
//    }
//
//    private List<HookTestStep> getBeforeStepHooks(List<PickleTag> tags) {
//        List<HookTestStep> hookSteps = new ArrayList<HookTestStep>();
//        add_test_steps_for_hooks(hookSteps, tags, glue.getBeforeStepHooks(), HookType.BeforeStep);
//        return hookSteps;
//    }
//
//    private void buildBackendWorlds() {
//        runtimeOptions.getPlugins(); // To make sure that the plugins are instantiated after
//        // the features have been parsed but before the pickles start to execute.
//        for (Backend backend : backends) {
//            backend.buildWorld();
//        }
//    }
//
//    private void disposeBackendWorlds() {
//        for (Backend backend : backends) {
//            backend.disposeWorld();
//        }
//    }
//}
