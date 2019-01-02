use std::fmt;

use {HookFn, CodeLocation};

/// Generated info for a hook definition
/// (for example a `#[before_scenario(...)]` annotated function).
pub struct StaticHookDefinition {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Execution order of the hook definition function of the same hook type. Higher
    pub order: isize,
    /// The generated step handler function responsible for calling the step definition function.
    pub hook_fn: HookFn,
    /// The generated step handler function responsible for calling the step definition function.
    pub location: CodeLocation,
}

impl fmt::Debug for StaticHookDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticHookDefinition")
            .field("name", &self.name)
            .field("hook_fn", &"<hook_fn>")
            .field("location", &self.location)
            .finish()
    }
}
