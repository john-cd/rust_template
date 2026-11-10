//! Configuration loading and application settings.
//!
//! Settings are layered, from lowest to highest priority:
//! built-in defaults, an optional TOML file, then `MY_LIB_*` environment
//! variables (for example `MY_LIB_NAME=Ada`).
//!
//! TODO Reviewed ??-??-2026.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;

/// Prefix of the environment variables that override file settings.
const ENV_PREFIX: &str = "MY_LIB";
/// Maximum configuration file size read into memory.
const MAX_CONFIG_FILE_BYTES: usize = 1024 * 1024;

/// Application settings. Replace the fields with your own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Name used when greeting.
    pub name: String,
    /// Whether to print extra diagnostic output.
    pub verbose: bool,
}

impl Default for Config {
    /// Returns the built-in settings used when no source overrides them.
    fn default() -> Self {
        Self {
            name: "world".to_owned(),
            verbose: false,
        }
    }
}

impl Config {
    /// Creates a configuration with the given name and other defaults.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Loads the configuration from defaults, the optional TOML `file`
    /// (a missing file is not an error), and `MY_LIB_*` environment variables.
    /// Files larger than 1 MiB are rejected before parsing.
    ///
    /// # Errors
    ///
    /// Returns an error if the file or environment values cannot be parsed.
    pub fn load(file: Option<&Path>) -> Result<Self> {
        Self::load_with_environment(file, config::Environment::with_prefix(ENV_PREFIX))
    }

    /// Loads settings from a file and the supplied environment source.
    ///
    /// Keeping the source injectable lets tests exercise configuration layering without reading
    /// unrelated `MY_LIB_*` values from the test runner's process environment.
    /// Loads settings from an explicitly supplied environment source for deterministic callers.
    fn load_with_environment(
        file: Option<&Path>,
        environment: config::Environment,
    ) -> Result<Self> {
        let mut builder = config::Config::builder();
        if let Some(path) = file
            && let Some(contents) = read_optional_config_file(path)?
        {
            let file_settings = toml::from_str::<Self>(&contents).map_err(|source| {
                crate::MyError::ConfigFileParse {
                    path: path.to_owned(),
                    source,
                }
            })?;
            let file_source = config::Config::try_from(&file_settings)?;
            builder = builder.add_source(file_source);
        }
        let settings = builder.add_source(environment).build()?;
        Ok(settings.try_deserialize()?)
    }

    /// Serializes the configuration to a TOML string.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }

    /// Saves the configuration to `path` as TOML, creating parent directories if needed.
    ///
    /// The complete serialized file is written using the library's atomic file replacement helper.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or writing the file fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = self.to_toml()?;
        crate::atomic_file::write_atomic(path, contents.as_bytes())
    }
}

/// Reads an optional TOML file without allowing an unbounded input allocation.
/// Reads an optional UTF-8 TOML file with a fixed upper bound on allocated input.
fn read_optional_config_file(path: &Path) -> Result<Option<String>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(crate::MyError::ConfigFileRead {
                path: path.to_owned(),
                source,
            });
        }
    };

    let mut bytes = Vec::with_capacity(MAX_CONFIG_FILE_BYTES + 1);
    file.take((MAX_CONFIG_FILE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| crate::MyError::ConfigFileRead {
            path: path.to_owned(),
            source,
        })?;
    if bytes.len() > MAX_CONFIG_FILE_BYTES {
        return Err(crate::MyError::ConfigFileTooLarge {
            path: path.to_owned(),
            limit_bytes: MAX_CONFIG_FILE_BYTES,
        });
    }

    String::from_utf8(bytes)
        .map(Some)
        .map_err(|source| crate::MyError::ConfigFileEncoding {
            path: path.to_owned(),
            source,
        })
}

/// Checks configuration defaults, source precedence, input limits, and persistence.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;

    use super::*;

    /// Builds a prefixed environment source without consulting the process environment.
    fn environment_source(values: &[(&str, &str)]) -> config::Environment {
        let source = values
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect::<config::Map<String, String>>();
        config::Environment::with_prefix(ENV_PREFIX).source(Some(source))
    }

    /// Returns a unique temporary directory path for one configuration test.
    fn test_directory(label: &str) -> PathBuf {
        /// Allocates distinct suffixes for tests running in parallel in this process.
        static NEXT_TEST_DIR_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "my_lib_config_{label}_{}_{}",
            std::process::id(),
            id
        ))
    }

    /// Saves settings to a nested path and loads them back unchanged.
    #[test]
    fn save_then_load_round_trips() -> Result<()> {
        let dir = test_directory("round_trip");
        let path = dir.join("nested").join("config.toml");
        let cfg = Config {
            name: "Ada".to_owned(),
            verbose: true,
        };
        cfg.save(&path)?;
        assert_eq!(
            Config::load_with_environment(Some(&path), environment_source(&[]))?,
            cfg
        );
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    /// Uses built-in settings when the optional configuration file is missing.
    #[test]
    fn missing_file_yields_defaults() -> Result<()> {
        let path = test_directory("missing").join("absent.toml");
        assert_eq!(
            Config::load_with_environment(Some(&path), environment_source(&[]))?,
            Config::default()
        );
        Ok(())
    }

    /// Applies supplied TOML fields while retaining defaults for omitted fields.
    #[test]
    fn partial_file_keeps_defaults() -> Result<()> {
        let dir = test_directory("partial");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("c.toml");
        std::fs::write(&path, "verbose = true\n")?;
        let cfg = Config::load_with_environment(Some(&path), environment_source(&[]))?;
        assert!(cfg.verbose);
        assert_eq!(cfg.name, "world");
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    /// Applies environment values after file values so they take precedence.
    #[test]
    fn environment_values_override_file_values() -> Result<()> {
        let dir = test_directory("environment");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        std::fs::write(&path, "name = \"file\"\nverbose = false\n")?;

        let config = Config::load_with_environment(
            Some(&path),
            environment_source(&[("MY_LIB_NAME", "environment"), ("MY_LIB_VERBOSE", "true")]),
        )?;

        assert_eq!(
            config,
            Config {
                name: "environment".to_owned(),
                verbose: true,
            }
        );
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    /// Reports parse failures with their paths for malformed TOML and invalid field types.
    #[test]
    fn malformed_file_error_includes_its_path() -> Result<()> {
        let dir = test_directory("load_error");
        std::fs::create_dir_all(&dir)?;
        let malformed = dir.join("malformed.toml");
        let invalid_type = dir.join("invalid_type.toml");
        std::fs::write(&malformed, "name = [\n")?;
        std::fs::write(&invalid_type, "name = { nested = true }\n")?;
        let error = Config::load_with_environment(Some(&malformed), environment_source(&[]))
            .expect_err("malformed TOML should be rejected");
        assert!(matches!(error, crate::MyError::ConfigFileParse { path, .. } if path == malformed));
        let error = Config::load_with_environment(Some(&invalid_type), environment_source(&[]))
            .expect_err("invalid field type should be rejected");
        assert!(
            matches!(error, crate::MyError::ConfigFileParse { path, .. } if path == invalid_type)
        );
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    /// Rejects oversized files after reading no more than the configured limit plus one byte.
    #[test]
    fn oversized_file_is_rejected_after_a_bounded_read() -> Result<()> {
        let dir = test_directory("oversized");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        let oversized_contents = vec![b' '; MAX_CONFIG_FILE_BYTES + 1];
        std::fs::write(&path, oversized_contents)?;

        let error = Config::load_with_environment(Some(&path), environment_source(&[]))
            .expect_err("oversized configuration should be rejected");
        assert!(matches!(
            error,
            crate::MyError::ConfigFileTooLarge { path: error_path, limit_bytes }
                if error_path == path && limit_bytes == MAX_CONFIG_FILE_BYTES
        ));
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
