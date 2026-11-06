//! Configuration loading and application settings.
//!
//! Settings are layered, from lowest to highest priority:
//! built-in defaults, an optional TOML file, then `MY_LIB_*` environment
//! variables (for example `MY_LIB_NAME=Ada`).
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;

/// Prefix of the environment variables that override file settings.
const ENV_PREFIX: &str = "MY_LIB";

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
    ///
    /// # Errors
    ///
    /// Returns an error if the file or environment values cannot be parsed.
    pub fn load(file: Option<&Path>) -> Result<Self> {
        let mut builder = config::Config::builder();
        if let Some(path) = file {
            builder = builder.add_source(
                config::File::from(path)
                    .format(config::FileFormat::Toml)
                    .required(false),
            );
        }
        let settings = builder
            .add_source(config::Environment::with_prefix(ENV_PREFIX))
            .build()?;
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

    /// Saves the configuration to `path` as TOML, creating parent
    /// directories if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or writing the file fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, self.to_toml()?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_then_load_round_trips() -> Result<()> {
        let dir = std::env::temp_dir().join(format!("my_lib_cfg_{}", std::process::id()));
        let path = dir.join("nested").join("config.toml");
        let cfg = Config {
            name: "Ada".to_owned(),
            verbose: true,
        };
        cfg.save(&path)?;
        assert_eq!(Config::load(Some(&path))?, cfg);
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn missing_file_yields_defaults() -> Result<()> {
        let path = std::env::temp_dir().join("my_lib_does_not_exist.toml");
        assert_eq!(Config::load(Some(&path))?, Config::default());
        Ok(())
    }

    #[test]
    fn partial_file_keeps_defaults() -> Result<()> {
        let dir = std::env::temp_dir().join(format!("my_lib_part_{}", std::process::id()));
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("c.toml");
        std::fs::write(&path, "verbose = true\n")?;
        let cfg = Config::load(Some(&path))?;
        assert!(cfg.verbose);
        assert_eq!(cfg.name, "world");
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
