/*!
This module is not intended for public use
and thus may break anytime (including patch updates).
*/

// TODO: Move to cuke_runner_codegen

use std::env;
use std::path::PathBuf;

#[macro_export]
macro_rules! cuke_runner {
    () => {
        #[test]
        fn cukes() {
            let mut tests_base_path = $crate::macros::__project_dir(); // -> /../../<crate>
            let current_file_path = file!(); // -> tests/cukes.rs
            tests_base_path.push(current_file_path); // -> /../../<crate>/tests/cukes.rs
            tests_base_path.pop(); // -> /../../<crate>/tests

            $crate::run_cukes(&tests_base_path);
        }
    };
}

//#[macro_export]
//#[doc(hidden)]
//macro_rules! __cuke_runner_include_steps {
//() => {
//        mod steps {
//            mod rpn_calculator {
//                include!("steps/rpn_calculator.rs");
//            }
//        }
//    };
//}

/// This function is not intended for public use
/// and thus may break anytime (including patch updates).
#[doc(hidden)]
pub fn __project_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            match env::current_dir() {
                Ok(cwd) => {
                    let cargo_toml = cwd.join("Cargo.toml");
                    if cargo_toml.exists() {
                        return Ok(cwd);
                    } else {
                        panic!("could not find Cargo.toml in current working directory: {}",
                            cwd.display());
                    }
                }
                Err(error) => return Err(error),
            }
        })
        .expect("could not determine current project directory")
}
