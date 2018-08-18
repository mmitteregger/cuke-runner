#![recursion_limit="128"]

extern crate cuke_runner;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate regex;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate log;

use proc_macro2::Span;
use proc_macro::TokenStream;
use std::env;
use syn::Path;

mod step_definitions;


//#[proc_macro]
//pub fn cuke_runner2(input: TokenStream) -> TokenStream {
////    fn __project_dir() -> ::std::path::PathBuf {
////        ::std::env::var("CARGO_MANIFEST_DIR")
////            .map(::std::path::PathBuf::from)
////            .or_else(|_| {
////                match ::std::env::current_dir() {
////                    Ok(cwd) => {
////                        let cargo_toml = cwd.join("Cargo.toml");
////                        if cargo_toml.exists() {
////                            return Ok(cwd);
////                        } else {
////                            panic!("could not find Cargo.toml in current working directory: {}",
////                                cwd.display());
////                        }
////                    }
////                    Err(error) => return Err(error),
////                }
////            })
////            .expect("could not determine current project directory")
////    }
//
////    let project_dir = __project_dir();
////    let mut file_path = ::std::path::PathBuf::from();
////
////    let mut glue_file_path: Option<::std::path::PathBuf> = None;
////    for common_prefix in file_path.ancestors() {
////        if (project_dir.ends_with(common_prefix)) {
////            let suffix = file_path.strip_prefix(common_prefix).unwrap();
////            glue_file_path = Some(project_dir.join(suffix));
////            break;
////        }
////    }
////
////    let glue_file_path = match glue_file_path {
////        Some(path) => path,
////        None => project_dir,
////    };
////
////    let glue_content = ::std::fs::read_to_string(&glue_file_path)
////        .expect(&format!("Cloud not read glue file: {}", file_path.display()));
////    println!("glue_content: {}", glue_content);
//    println!("input2: {}", input);
//    println!("input2: {}", ::std::env::var("CARGO_MANIFEST_DIR").unwrap());
//    println!("input2: {}", ::std::env::current_dir().unwrap().display());
////    TokenStream::new()
//
////    let call_site_span = Span::call_site();
////    let tokens = quote_spanned! {call_site_span=>
////        #[test]
////        fn cukes() {
////            let root_path = ::std::path::PathBuf::from(
////                r"\\?\C:\Users\Michael\IdeaProjects\cuke-runner\examples\calculator\tests");
////            debug_assert!(root_path . exists (  ) ,
////                          "expected root path \"{}\" to exist" , root_path . display (
////                           ));
////
////            let glue = ::cuke_runner::codegen::Glue::from_static_step_definitions(&[
////                    &::steps::rpn_calculator::static_step_definition_for_reset_calculator,
////                    &::steps::rpn_calculator::static_step_definition_for_add,
////                    &::steps::rpn_calculator::static_step_definition_for_press,
////                    &::steps::rpn_calculator::static_step_definition_for_assert_result
////            ]);
////
////            ::cuke_runner::run_cukes(root_path, glue);
////        }
////    };
////    tokens.into()
//    input
//}

#[proc_macro]
pub fn step_definitions(input: TokenStream) -> TokenStream {
    let create_root_path = env::current_dir()
        .expect("current directory for crate root path");
    let crate_relative_path = env::args()
        .find(|arg| arg.ends_with(".rs"))
        .expect("could not find compiling rust file in current argument list");
    let mut current_file_path = create_root_path.join(crate_relative_path);

    println!("current_file_path: {}", current_file_path.display());
    let step_definition_paths = step_definitions::parse(&mut current_file_path);
    println!("step_definition_paths: {:?}", step_definition_paths);

    let step_definition_path_tokens = step_definition_paths.into_iter()
        .map(|step_definition_path| {
            syn::parse_str::<Path>(&step_definition_path).expect("parse step definition paths")
        })
        .collect::<Vec<_>>();

    let call_site_span = Span::call_site();
    let step_definition_tokens = quote_spanned! {call_site_span=>
        pub static STEP_DEFINITIONS: &[&::cuke_runner::codegen::StaticStepDefinition] = &[
            #(&#step_definition_path_tokens,
            )*
        ];
    };

    TokenStream::from(step_definition_tokens)
}

//#[proc_macro_derive(Glue)]
//pub fn derive_glue(input: TokenStream) -> TokenStream {
////    let input: DeriveInput = syn::parse(input).unwrap();
//    let i = quote! {
//    };
//    i.into()
//}
//
//fn compile_error(message: String) -> proc_macro2::TokenStream {
//    quote! {
//        compile_error!(#message);
//    }
//}
//
//#[macro_export]
//macro_rules! generate_glue {
//    () => (
//        generate_glue_internal!(file!());
//        #[test]
//        fn generate_glue() {
//            let project_dir = ::cuke_runner_derive::__project_dir();
//            let mut file_path = ::std::path::PathBuf::from(file!());
//
//            let mut glue_file_path: Option<::std::path::PathBuf> = None;
//            for common_prefix in file_path.ancestors() {
//                if (project_dir.ends_with(common_prefix)) {
//                    let suffix = file_path.strip_prefix(common_prefix).unwrap();
//                    glue_file_path = Some(project_dir.join(suffix));
//                    break;
//                }
//            }
//
//            let glue_file_path = match glue_file_path {
//                Some(path) => path,
//                None => project_dir,
//            };

//            let glue_content = ::std::fs::read_to_string(&glue_file_path)
//                    .expect(&format!("Cloud not read glue file: {}", file_path.display()));
//            println!("glue_content: {}", glue_content);
//        }
//        ::cuke_runner_derive::__project_dir();
//        pub static GLUE: ::cuke_runner::codegen::Glue =
//                ::cuke_runner::codegen::Glue::from_static_step_definitions(&[
//
//                ]);
//    )
//}

//#[doc(hidden)]
//pub fn __project_dir() -> ::std::path::PathBuf {
//    ::std::env::var("CARGO_MANIFEST_DIR")
//        .map(::std::path::PathBuf::from)
//        .or_else(|_| {
//            match ::std::env::current_dir() {
//                Ok(cwd) => {
//                    let cargo_toml = cwd.join("Cargo.toml");
//                    if cargo_toml.exists() {
//                        return Ok(cwd);
//                    } else {
//                        panic!("could not find Cargo.toml in current working directory: {}",
//                            cwd.display());
//                    }
//                }
//                Err(error) => return Err(error),
//            }
//        })
//        .expect("could not determine current project directory")
//}
