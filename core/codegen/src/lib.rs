#![recursion_limit="128"]
#![warn(rust_2018_idioms)]

use cuke_runner_glue as glue;

use devise;
use proc_macro::TokenStream;

use crate::glue::hook::HookType;
use crate::glue::step::StepKeyword;

#[macro_use]
mod proc_macro_ext;
mod attribute;
mod bang;
mod glue_codegen;
mod syn_ext;

static BEFORE_SCENARIO_HOOK_STRUCT_PREFIX: &str = "static_cuke_runner_before_scenario_hook_info_for_";
static BEFORE_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX: &str = "cuke_runner_before_scenario_hook_fn_location_fn_";
static BEFORE_SCENARIO_HOOK_FN_PREFIX: &str = "cuke_runner_before_scenario_hook_fn_";
static BEFORE_STEP_HOOK_STRUCT_PREFIX: &str = "static_cuke_runner_before_step_hook_info_for_";
static BEFORE_STEP_HOOK_FN_LOCATION_FN_PREFIX: &str = "cuke_runner_before_step_hook_fn_location_fn_";
static BEFORE_STEP_HOOK_FN_PREFIX: &str = "cuke_runner_before_step_hook_fn_";
static AFTER_STEP_HOOK_STRUCT_PREFIX: &str = "static_cuke_runner_after_step_hook_info_for_";
static AFTER_STEP_HOOK_FN_LOCATION_FN_PREFIX: &str = "cuke_runner_after_step_hook_fn_location_fn_";
static AFTER_STEP_HOOK_FN_PREFIX: &str = "cuke_runner_after_step_hook_fn_";
static AFTER_SCENARIO_HOOK_STRUCT_PREFIX: &str = "static_cuke_runner_after_scenario_hook_info_for_";
static AFTER_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX: &str = "cuke_runner_after_scenario_hook_fn_location_fn_";
static AFTER_SCENARIO_HOOK_FN_PREFIX: &str = "cuke_runner_after_scenario_hook_fn_";

static STEP_STRUCT_PREFIX: &str = "static_cuke_runner_step_info_for_";
static STEP_FN_LOCATION_FN_PREFIX: &str = "cuke_runner_step_fn_location_fn_";
static STEP_FN_PREFIX: &str = "cuke_runner_step_fn_";

static PARAM_PREFIX: &str = "__cuke_runner_param_";

macro_rules! emit {
    ($tokens:expr) => ({
        use devise::ext::SpanDiagnosticExt;

        let mut tokens = $tokens;
        if let Ok(debug) = ::std::env::var("CUKE_CODEGEN_DEBUG") {
            if &debug == "true" {
                let debug_tokens = proc_macro2::Span::call_site()
                    .note("emitting cuke runner code generation debug output")
                    .note(tokens.to_string())
                    .emit_as_item_tokens();

                tokens.extend(debug_tokens);
            }
        }

        tokens.into()
    })
}

macro_rules! hook_attribute {
    ($name:ident => $hook_type:expr) => (
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, input: TokenStream) -> TokenStream {
            emit!(attribute::hook::hook_attribute($hook_type, args, input))
        }
    )
}

hook_attribute!(hook => None);
hook_attribute!(before_scenario => HookType::BeforeScenario);
hook_attribute!(before_step => HookType::BeforeStep);
hook_attribute!(after_step => HookType::AfterStep);
hook_attribute!(after_scenario => HookType::AfterScenario);

macro_rules! step_attribute {
    ($name:ident => $keyword:expr) => (
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, input: TokenStream) -> TokenStream {
            emit!(attribute::step::step_attribute($keyword, args, input))
        }
    )
}

step_attribute!(step => StepKeyword::Star);
step_attribute!(given => StepKeyword::Given);
step_attribute!(when => StepKeyword::When);
step_attribute!(then => StepKeyword::Then);


#[proc_macro]
pub fn generate_glue(input: TokenStream) -> TokenStream {
    emit!(bang::generate_glue_macro(input))
}

#[proc_macro]
pub fn glue(input: TokenStream) -> TokenStream {
    emit!(bang::glue_macro(input))
}
