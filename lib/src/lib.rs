/*!
Cucumber for Rust with a focus on ease-of-use.
*/

extern crate gherkin;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate walkdir;
extern crate state;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate downcast_rs;

use std::path::Path;

use gherkin::ast::*;

pub use data::State;

use config::CukeConfig;
pub use error::{Result, Error};

mod config;
mod error;
#[doc(hidden)]
pub mod codegen;
pub mod data;
mod parser;
mod api;
mod runner;
mod runtime;

pub fn run_cukes<P: AsRef<Path>>(tests_base_path: P) {
    match run(tests_base_path) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Uh oh, looks like some cukes have rotten: {}", error);
            ::std::process::exit(-1);
        },
    };
}

fn run<P: AsRef<Path>>(tests_base_path: P) -> Result<()> {
    let config = CukeConfig::read(tests_base_path)?;
    let gherkin_documents = parser::parse_gherkin_documents(&config.features)?;

    gherkin_documents.into_iter()
        .for_each(|gherkin_document| {
            let feature = match gherkin_document.get_feature() {
                Some(feature) => feature,
                None => return,
            };

            println!("Feature: {}\n", feature.get_name());
            let scenario_definitions = feature.get_children();

            let mut background_printed = false;
            let mut scenario_printed = false;
            for scenario_definition in scenario_definitions {
                if let Some(background) = scenario_definition.downcast_ref::<Background>() {
                    if !background_printed {
                        println!("  Background:");
                        background_printed = true;
                    }

                    println!("    {}", background.get_name());
                } else if let Some(scenario) = scenario_definition.downcast_ref::<Scenario>() {
                    if !scenario_printed {
                        println!("  Scenario:");
                        scenario_printed = true;
                    }

                    println!("    {}", scenario.get_name());
                } else if let Some(scenario_outline) = scenario_definition.downcast_ref::<ScenarioOutline>() {
                    // unimplemented!();
                } else {
                    panic!("Unexpected scenario definition: {:?}", scenario_definition);
                }
            }
        });

    Ok(())
}
