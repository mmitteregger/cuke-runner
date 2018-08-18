pub use self::event_bus::EventBus;
pub use self::test_step::{HookTestStep, PickleStepTestStep};

mod event_bus;
mod test_step;
pub mod util;

use gherkin::event::PickleEvent;
use gherkin::pickle::PickleTag;

use Config;
use runtime::{Glue, HookDefinition};
use api::HookType;
use runtime::{TestCase, HookDefinitionMatch};

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

    pub fn run_pickle(&self, pickle_event: PickleEvent, event_bus: &EventBus) {
        let test_case = self.create_test_case_for_pickle_event(pickle_event);
        test_case.run(event_bus);
    }

    fn create_test_case_for_pickle_event(&self, pickle_event: PickleEvent) -> TestCase {
        let (
            before_hooks,
            test_steps,
            after_hooks
        ) = if pickle_event.pickle.get_steps().is_empty() {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        } else {
            (
                self.create_before_scenario_hooks(pickle_event.pickle.get_tags()),
                self.create_test_steps(&pickle_event),
                self.create_after_scenario_hooks(pickle_event.pickle.get_tags()),
            )
        };

        TestCase {
            test_steps,
            before_hooks,
            after_hooks,
            pickle_event,
            dry_run: self.dry_run,
        }
    }

    fn create_before_scenario_hooks(&self, tags: &Vec<PickleTag>) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_before_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::BeforeScenario)
    }

    fn create_after_scenario_hooks(&self, tags: &Vec<PickleTag>) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_after_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::AfterScenario)
    }

    fn create_hooks(&self, tags: &Vec<PickleTag>, hook_definitions: &Vec<HookDefinition>,
        hook_type: HookType) -> Vec<HookTestStep>
    {
        let mut hooks = Vec::with_capacity(hook_definitions.len());

        for hook_definition in hook_definitions {
            if hook_definition.matches(tags) {
                let test_step = HookTestStep {
                    definition_match: HookDefinitionMatch { hook_definition: hook_definition.clone() },
                    hook_type,
                };
                hooks.push(test_step);
            }
        }

        hooks
    }

    fn create_test_steps(&self, pickle_event: &PickleEvent) -> Vec<PickleStepTestStep> {
        unimplemented!();

        for step in pickle_event.pickle.get_steps() {

        };

        Vec::new()
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
//            List<HookTestStep> afterStepHookSteps = getAfterStepHooks(pickleEvent.pickle.getTags());
//            List<HookTestStep> beforeStepHookSteps = getBeforeStepHooks(pickleEvent.pickle.getTags());
//            testSteps.add(new PickleStepTestStep(pickleEvent.uri, step, beforeStepHookSteps, afterStepHookSteps, match));
//        }
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
