use std::collections::HashMap;
use std::sync::Arc;

use gherkin::pickle::PickleStep;

use glue::StaticStepDefinition;
use runtime::{
    Scenario,
    AmbiguousPickleStepDefinitionMatch, HookDefinition, PickleStepDefinitionMatch,
    StepDefinition, StepDefinitionMatch, UndefinedPickleStepDefinitionMatch,
};
use runtime::step_expression::StepExpression;
use api::SourceCodeLocation;

#[derive(Debug)]
pub struct Glue {
    step_definitions_by_pattern: HashMap<&'static str, StepDefinition>,
    before_scenario_hooks: Vec<HookDefinition>,
    before_step_hooks: Vec<HookDefinition>,
    after_step_hooks: Vec<HookDefinition>,
    after_scenario_hooks: Vec<HookDefinition>,
}

impl From<&'static [&'static StaticStepDefinition]> for Glue {
    fn from(step_definitions: &'static [&'static StaticStepDefinition]) -> Glue {
        let mut step_definitions_by_pattern =
            HashMap::with_capacity(step_definitions.len());

        for step_definition in step_definitions {
            let definition = StepDefinition {
                expression: StepExpression::from_regex(step_definition.expression),
                parameter_infos: Vec::new(),
                step_fn: step_definition.step_fn,
                location: SourceCodeLocation {
                    file_path: String::new(),
                    line_number: 0,
                },
            };

            step_definitions_by_pattern.insert(step_definition.expression, definition);
        }

        Glue {
            step_definitions_by_pattern,
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }
}

impl Glue {
    pub fn new() -> Glue {
        Glue {
            step_definitions_by_pattern: HashMap::new(),
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }

    pub fn from_static_step_definitions(_static_step_definitions: &[&StaticStepDefinition]) -> Glue {
        Glue {
            step_definitions_by_pattern: HashMap::new(),
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }

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
            });
        }
        if matches.len() > 1 {
            return Box::new(AmbiguousPickleStepDefinitionMatch {
                feature_path: feature_path.clone(),
                step,
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
