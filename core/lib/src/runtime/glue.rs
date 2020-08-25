use std::collections::HashMap;

use gherkin::cuke;

use crate::glue::StaticGlueDefinitions;
use crate::runtime::{
    AmbiguousCukeStepDefinitionMatch, HookDefinition, CukeStepDefinitionMatch,
    StepDefinition, StepDefinitionMatch, UndefinedCukeStepDefinitionMatch,
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Glue {
    before_scenario_hooks: Vec<HookDefinition>,
    before_step_hooks: Vec<HookDefinition>,
    step_definitions_by_pattern: HashMap<&'static str, StepDefinition>,
    after_step_hooks: Vec<HookDefinition>,
    after_scenario_hooks: Vec<HookDefinition>,
}

#[doc(hidden)]
impl From<(PathBuf, &[StaticGlueDefinitions])> for Glue {
    fn from((base_path, static_glue_definitions): (PathBuf, &[StaticGlueDefinitions])) -> Glue {
        let base_path = base_path.as_path();

        let before_scenario_hooks = static_glue_definitions.iter()
            .flat_map(|glue| glue.before_scenario_hooks.iter())
            .map(|static_hook_def| HookDefinition::from((base_path, static_hook_def)))
            .collect();
        let before_step_hooks = static_glue_definitions.iter()
            .flat_map(|glue| glue.before_step_hooks.iter())
            .map(|static_hook_def| HookDefinition::from((base_path, static_hook_def)))
            .collect();
        let after_step_hooks = static_glue_definitions.iter()
            .flat_map(|glue| glue.after_step_hooks.iter())
            .map(|static_hook_def| HookDefinition::from((base_path, static_hook_def)))
            .collect();
        let after_scenario_hooks = static_glue_definitions.iter()
            .flat_map(|glue| glue.after_scenario_hooks.iter())
            .map(|static_hook_def| HookDefinition::from((base_path, static_hook_def)))
            .collect();

        let step_definitions_capacity = static_glue_definitions.iter()
            .flat_map(|glue| glue.steps.iter())
            .count();
        let mut step_definitions_by_pattern = HashMap::with_capacity(step_definitions_capacity);
        static_glue_definitions.iter()
            .flat_map(|glue| glue.steps.iter())
            .map(|static_step_definition|
                (static_step_definition.expression, StepDefinition::from((base_path, static_step_definition))))
            .for_each(|(expression, step_definition)| {
                let new_location = step_definition.location.clone();

                if let Some(prev) = step_definitions_by_pattern.insert(expression, step_definition) {
                    let prev_location = prev.location;
                    panic!("duplicate step definition \"{}\":
   first: {}
  second: {}", prev.expression.regex.as_str(), prev_location, new_location)
                }
            });

        Glue {
            before_scenario_hooks,
            before_step_hooks,
            step_definitions_by_pattern,
            after_step_hooks,
            after_scenario_hooks,
        }
    }
}

#[doc(hidden)]
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

    pub fn step_definition_match<'s, 'a: 's>(&'a self, feature_path: &str, step: &'s cuke::Step<'_>)
        -> StepDefinitionMatch<'s> {

        let mut matches = Vec::new();

        for step_definition in self.step_definitions_by_pattern.values() {
            let arguments = step_definition.matched_arguments(&step);

            if let Some(arguments) = arguments {
                matches.push(CukeStepDefinitionMatch {
                    arguments,
                    step_definition: step_definition.clone(),
                    feature_path: feature_path.to_owned(),
                    step,
                });
            }
        }

        if matches.is_empty() {
            return StepDefinitionMatch::Undefined(UndefinedCukeStepDefinitionMatch {
                step,
                arguments: Vec::new(),
            });
        }
        if matches.len() > 1 {
            return StepDefinitionMatch::Ambiguous(AmbiguousCukeStepDefinitionMatch {
                feature_path: feature_path.to_owned(),
                step,
                arguments: Vec::new(),
            });
        }

        let step_definition_match = matches.pop().unwrap();
        StepDefinitionMatch::Cuke(step_definition_match)
    }

}
