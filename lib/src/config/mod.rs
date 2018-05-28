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
pub struct CukeConfig {
    pub features: PathBuf,
    pub output: PathBuf,
    pub strict: bool,
    pub monochrome: bool,
    pub tags: Vec<String>,
    pub parallel_scheme: ParallelScheme,
}

/// Controls how the cucumber tests are executed in parallel.
#[derive(Debug)]
// TODO: Use...
#[allow(unused)]
pub enum ParallelScheme {
    /// Execute all scenarios in parallel.
    Scenario,
    /// Execute all features in parallel, but not their scenarios.
    Feature,
    /// Execute every scenario one after another (no parallelism).
    Sequential,
}

impl Default for ParallelScheme {
    fn default() -> ParallelScheme {
        ParallelScheme::Scenario
    }
}

impl CukeConfig {
    pub fn read<P: AsRef<Path>>(base_path: P) -> Result<CukeConfig> {
        let path: PathBuf = match CukeConfig::find(&base_path) {
            Ok(path) => path,
            Err(error) => match error {
                Error::NotFound => return CukeConfig::create_default(&base_path),
                _ => return Err(error),
            }
        };

        let toml_config = fs::read_to_string(&path)?;

        CukeConfig::parse(toml_config, &path)
    }

    /// Iteratively search for `CONFIG_FILENAME` starting at the given base path
    /// and working up through its parents. Returns the path to the
    /// file or an `Error::NotFound` if the file couldn't be found.
    fn find<P: AsRef<Path>>(base_path: P) -> Result<PathBuf> {
        let mut current = base_path.as_ref();

        loop {
            let manifest = current.join(CONFIG_FILENAME);
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest)
            }

            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Err(Error::NotFound)
    }

    fn create_default<P: AsRef<Path>>(base_path: P) -> Result<CukeConfig> {
        let base_path = base_path.as_ref();
        let cargo_toml_dir = PathBuf::from(get_env_var("CARGO_MANIFEST_DIR")?);
        let cargo_out_dir = cargo_toml_dir.join("target");

        let config = CukeConfig {
            features: base_path.join("features"),
            output: cargo_out_dir.join("cukes"),
            strict: false,
            monochrome: false,
            tags: Vec::new(),
            parallel_scheme: ParallelScheme::default(),
        };
        return Ok(config);
    }

    fn parse<P: AsRef<Path>>(_src: String, path: P) -> Result<CukeConfig> {
        let base_path = path.as_ref().join("tests");
        let cargo_out_dir = get_env_var("CARGO_MANIFEST_DIR")?;

        let config = CukeConfig {
            features: base_path.join("features"),
            output: PathBuf::from(cargo_out_dir).join("target").join("cukes"),
            strict: false,
            monochrome: false,
            tags: Vec::new(),
            parallel_scheme: ParallelScheme::default(),
        };
        Ok(config)
    }
}

fn get_env_var(key: &'static str) -> Result<String> {
    env::var(key).map_err(|error| Error::EnvVar(error, key))
}
