use std::collections::HashMap;
use std::sync::Arc;

use gherkin::pickle::PickleStep;

use glue::StaticGlueDefinitions;
use runtime::{
    AmbiguousPickleStepDefinitionMatch, HookDefinition, PickleStepDefinitionMatch,
    StepDefinition, StepDefinitionMatch, UndefinedPickleStepDefinitionMatch,
};

#[derive(Debug)]
pub struct Glue {
    before_scenario_hooks: Vec<HookDefinition>,
    before_step_hooks: Vec<HookDefinition>,
    step_definitions_by_pattern: HashMap<&'static str, StepDefinition>,
    after_step_hooks: Vec<HookDefinition>,
    after_scenario_hooks: Vec<HookDefinition>,
}

impl From<StaticGlueDefinitions> for Glue {
    fn from(static_glue_definitions: StaticGlueDefinitions) -> Glue {
        let before_scenario_hooks = static_glue_definitions.before_scenario_hooks.into_iter()
            .map(HookDefinition::from)
            .collect();
        let before_step_hooks = static_glue_definitions.before_step_hooks.into_iter()
            .map(HookDefinition::from)
            .collect();
        let step_definitions_by_pattern = static_glue_definitions.steps.into_iter()
            .map(|static_step_definition|
                (static_step_definition.expression, StepDefinition::from(static_step_definition)))
            .collect();
        let after_step_hooks = static_glue_definitions.after_step_hooks.into_iter()
            .map(HookDefinition::from)
            .collect();
        let after_scenario_hooks = static_glue_definitions.after_scenario_hooks.into_iter()
            .map(HookDefinition::from)
            .collect();

        Glue {
            before_scenario_hooks,
            before_step_hooks,
            step_definitions_by_pattern,
            after_step_hooks,
            after_scenario_hooks,
        }
    }
}

impl Glue {
    pub fn get_step_definitions_by_pattern(&self) -> &HashMap<&'static str, StepDefinition> {
        &self.step_definitions_by_pattern
    }

    pub fn get_before_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_scenario_hooks
    }

    pub fn get_before_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_step_hooks
    }

    pub fn get_after_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_step_hooks
    }

    pub fn get_after_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_scenario_hooks
    }

    pub fn step_definition_match(&self, feature_path: &String, step: PickleStep)
        -> Box<StepDefinitionMatch> {

        let step = Arc::new(step);

        let mut matches = self.step_definition_matches(feature_path, step.clone());

        if matches.is_empty() {
            return Box::new(UndefinedPickleStepDefinitionMatch {
                step,
                arguments: Vec::new(),
            });
        }
        if matches.len() > 1 {
            return Box::new(AmbiguousPickleStepDefinitionMatch {
                feature_path: feature_path.clone(),
                step,
                arguments: Vec::new(),
            });
        }

        let step_definition_match = matches.pop().unwrap();
        Box::new(step_definition_match)
    }

    pub fn step_definition_matches(&self, feature_path: &String, step: Arc<PickleStep>)
        -> Vec<PickleStepDefinitionMatch> {

        let mut matches = Vec::new();

        for step_definition in self.step_definitions_by_pattern.values() {
            let arguments = step_definition.matched_arguments(&step);

            if let Some(arguments) = arguments {
                matches.push(PickleStepDefinitionMatch {
                    arguments,
                    step_definition: step_definition.clone(),
                    feature_path: feature_path.clone(),
                    step: step.clone(),
                });
            }
        }

        matches
    }
}
