use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::default::Default;

pub use self::error::Error;

use self::error::Result;

mod error;

const CONFIG_FILENAME: &'static str = "Cukes.toml";
// TODO: Use ENV_VAR_PREFIX
const _ENV_VAR_PREFIX: &'static str = "CUKE_";

#[derive(Debug)]
pub struct Config {
    pub features_dir: PathBuf,
    pub output_dir: PathBuf,
    pub strict: bool,
    pub colored_output: bool,
    pub dry_run: bool,
    pub tags: Vec<String>,
    pub execution_mode: ExecutionMode,
}

/// Controls how the cucumber tests are executed.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
// TODO: Use...
#[allow(unused)]
pub enum ExecutionMode {
    /// Execute all scenarios in parallel.
    ParallelScenarios,
    /// Execute all features in parallel, but not scenarios from the same feature.
    ParallelFeatures,
    /// Execute every scenario one after another (no parallelism).
    Sequential,
}

impl Default for ExecutionMode {
    fn default() -> ExecutionMode {
        ExecutionMode::ParallelScenarios
    }
}

impl Config {
//    pub fn read<P: AsRef<Path>>(base_path: P) -> Result<Config> {
//        let path: PathBuf = match Config::find(&base_path) {
//            Ok(path) => path,
//            Err(error) => match error {
//                Error::NotFound => return Config::create_default(&base_path),
//                _ => return Err(error),
//            }
//        };
//
//        let toml_config = fs::read_to_string(&path)?;
//
//        Config::parse(toml_config, &path)
//    }
//
//    /// Iteratively search for `CONFIG_FILENAME` starting at the given base path
//    /// and working up through its parents. Returns the path to the
//    /// file or an `Error::NotFound` if the file couldn't be found.
//    fn find<P: AsRef<Path>>(base_path: P) -> Result<PathBuf> {
//        let mut current = base_path.as_ref();
//
//        loop {
//            let manifest = current.join(CONFIG_FILENAME);
//            if fs::metadata(&manifest).is_ok() {
//                return Ok(manifest)
//            }
//
//            match current.parent() {
//                Some(p) => current = p,
//                None => break,
//            }
//        }
//
//        Err(Error::NotFound)
//    }
//
//    fn create_default<P: AsRef<Path>>(base_path: P) -> Result<Config> {
//        let base_path = base_path.as_ref();
//        let cargo_toml_dir = PathBuf::from(get_env_var("CARGO_MANIFEST_DIR")?);
//        let cargo_out_dir = cargo_toml_dir.join("target");
//
//        let config = Config {
//            features_dir: base_path.join("features"),
//            output_dir: cargo_out_dir.join("cukes"),
//            strict: false,
//            colored_output: false,
//            tags: Vec::new(),
//            parallel_scheme: ParallelScheme::default(),
//        };
//        return Ok(config);
//    }
//
//    fn parse<P: AsRef<Path>>(_src: String, path: P) -> Result<Config> {
//        let base_path = path.as_ref().join("tests");
//        let cargo_out_dir = get_env_var("CARGO_MANIFEST_DIR")?;
//
//        let config = Config {
//            features_dir: base_path.join("features"),
//            output_dir: PathBuf::from(cargo_out_dir).join("target").join("cukes"),
//            strict: false,
//            colored_output: false,
//            tags: Vec::new(),
//            parallel_scheme: ParallelScheme::default(),
//        };
//        Ok(config)
//    }
}

//fn get_env_var(key: &'static str) -> Result<String> {
//    env::var(key).map_err(|error| Error::EnvVar(error, key))
//}
