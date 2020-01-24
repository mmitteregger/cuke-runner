pub use self::event::EventPublisher;
pub(crate) use self::event::{EventBus, SyncEventBus};
pub use self::test_step::{HookTestStep, CukeStepTestStep};

mod event;
mod test_step;

use gherkin::cuke::{Cuke, Tag};

use api::HookType;
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

    pub fn run<EP: EventPublisher>(&self, uri: &str, cuke: Cuke, event_publisher: &EP) {
        let test_case = self.create_test_case(uri, &cuke);
        runtime::test_case::run(test_case, event_publisher);
    }

    fn create_test_case<'c, 's: 'c>(&'s self, uri: &'c str, cuke: &'c Cuke) -> TestCase<'c> {
        let (
            before_hooks,
            after_hooks,
            test_steps,
        ) = if cuke.scenario_steps.is_empty() {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        } else {
            let tags = &cuke.tags;
            (
                self.create_before_scenario_hooks(tags),
                self.create_after_scenario_hooks(tags),
                self.create_test_steps(uri, cuke),
            )
        };

        TestCase {
            uri,
            cuke,
            test_steps,
            before_hooks,
            after_hooks,
            dry_run: self.dry_run,
        }
    }

    fn create_before_scenario_hooks(&self, tags: &[Tag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_before_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::BeforeScenario)
    }

    fn create_after_scenario_hooks(&self, tags: &[Tag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_after_scenario_hooks();
        self.create_hooks(tags, hook_definitions, HookType::AfterScenario)
    }

    fn create_hooks(&self, tags: &[Tag], hook_definitions: &[HookDefinition],
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

    fn create_test_steps<'e, 's: 'e>(&'s self, uri: &'s str, cuke: &'e Cuke)
        -> Vec<CukeStepTestStep<'e>>
    {
        let mut test_steps = Vec::new();

        let feature_path = uri;
        let tags = &cuke.tags;

        for step in &cuke.feature_background_steps {
            let step_definition_match = self.glue.step_definition_match(feature_path, step);
            let before_step_hook_steps = self.get_before_step_hooks(tags);
            let after_step_hook_steps = self.get_after_step_hooks(tags);

            test_steps.push(CukeStepTestStep {
                uri: feature_path.to_string(),
                before_step_hook_steps,
                after_step_hook_steps,
                step_definition_match,
                background_step: true,
            });
        }
        for step in &cuke.rule_background_steps {
            let step_definition_match = self.glue.step_definition_match(feature_path, step);
            let before_step_hook_steps = self.get_before_step_hooks(tags);
            let after_step_hook_steps = self.get_after_step_hooks(tags);

            test_steps.push(CukeStepTestStep {
                uri: feature_path.to_string(),
                before_step_hook_steps,
                after_step_hook_steps,
                step_definition_match,
                background_step: true,
            });
        }
        for step in &cuke.scenario_steps {
            let step_definition_match = self.glue.step_definition_match(feature_path, step);
            let before_step_hook_steps = self.get_before_step_hooks(tags);
            let after_step_hook_steps = self.get_after_step_hooks(tags);

            test_steps.push(CukeStepTestStep {
                uri: feature_path.to_string(),
                before_step_hook_steps,
                after_step_hook_steps,
                step_definition_match,
                background_step: false,
            });
        }

        test_steps
    }

    fn get_before_step_hooks(&self, tags: &[Tag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_before_step_hooks();
        self.create_hooks(tags, hook_definitions, HookType::BeforeStep)
    }

    fn get_after_step_hooks(&self, tags: &[Tag]) -> Vec<HookTestStep> {
        let hook_definitions = &self.glue.get_after_step_hooks();
        self.create_hooks(tags, hook_definitions, HookType::AfterStep)
    }
}
