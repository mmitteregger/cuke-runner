use std::fmt;
use std::path::{Path, PathBuf};

use gherkin::cuke::Tag;

use crate::api::GlueCodeLocation;
use crate::error::Result;
use crate::glue::filter::tag::TagPredicate;
use crate::glue::hook::{HookFn, StaticHookDef};
use crate::runtime::Scenario;

#[derive(Clone)]
pub struct HookDefinition {
    tag_predicate: TagPredicate<'static>,
    order: u32,
    //    timeout: Option<Duration>,
    hook_fn: HookFn,
    location: GlueCodeLocation,
}

impl fmt::Debug for HookDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("HookDefinition")
            .field("tag_predicate", &self.tag_predicate)
            .field("order", &self.order)
//            .field("timeout", &self.timeout)
            .field("hook_fn", &"<hook_fn>")
            .field("location", &self.location)
            .finish()
    }
}

impl From<(&Path, &&StaticHookDef)> for HookDefinition {
    fn from((base_path, static_hook_def): (&Path, &&StaticHookDef)) -> Self {
        let tag_predicate = TagPredicate::new(static_hook_def.tag_expression)
            .unwrap_or_else(|err| {
                panic!("tag expression \"{}\"is invalid: {}\n\
                        It should have been checked by codegen already, this is a bug!",
                    static_hook_def.tag_expression, err);
            });

        let location = (static_hook_def.hook_fn_location_fn)();
        let file_path = PathBuf::from(location.file);
        let mut relative_file_path = file_path.as_path();
        for ancestor in file_path.ancestors() {
            if base_path.ends_with(ancestor) {
                match file_path.strip_prefix(ancestor) {
                    Ok(path) => {
                        relative_file_path = path;
                        break;
                    },
                    Err(_strip_prefix_error) => {
                        panic!("unable to strip base path \"{}\" from path \"{}\"",
                               base_path.display(), file_path.display());
                    }
                }
            }
        }

        HookDefinition {
            tag_predicate,
            order: 0,
            hook_fn: static_hook_def.hook_fn,
            location: GlueCodeLocation {
                file_path: relative_file_path.to_owned(),
                line_number: location.line,
            },
        }
    }
}

impl HookDefinition {
    pub fn get_location(&self) -> &GlueCodeLocation {
        &self.location
    }

    pub fn execute(&self, scenario: &mut Scenario<'_, '_>) -> Result<()> {
        let result = (self.hook_fn)(&mut scenario.glue_scenario);
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(crate::error::Error::Execution(error)),
        }
    }

    pub fn matches(&self, tags: &[Tag<'_>]) -> bool {
        self.tag_predicate.apply(tags)
    }

    pub fn get_order(&self) -> u32 {
        self.order
    }
}
