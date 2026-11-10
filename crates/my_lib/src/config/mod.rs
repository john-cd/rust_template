//! Configuration loading and application settings.
//!
//! Settings are layered, from lowest to highest priority:
//! built-in defaults, an optional TOML file, then `MY_LIB_*` environment
//! variables (for example `MY_LIB_NAME=Ada`).
//!
//! TODO Reviewed ??-??-2026.

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;

/// Prefix of the environment variables that override file settings.
const ENV_PREFIX: &str = "MY_LIB";
/// Process-local counter used to avoid collisions between concurrent saves.
static NEXT_TEMP_FILE_ID: AtomicU64 = AtomicU64::new(0);

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
        Self::load_with_environment(file, config::Environment::with_prefix(ENV_PREFIX))
    }

    /// Loads settings from a file and the supplied environment source.
    ///
    /// Keeping the source injectable lets tests exercise configuration layering without reading
    /// unrelated `MY_LIB_*` values from the test runner's process environment.
    fn load_with_environment(
        file: Option<&Path>,
        environment: config::Environment,
    ) -> Result<Self> {
        let mut builder = config::Config::builder();
        if let Some(path) = file {
            builder = builder.add_source(
                config::File::from(path)
                    .format(config::FileFormat::Toml)
                    .required(false),
            );
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
    /// The complete serialized file is written to a uniquely named sibling and synced before it
    /// replaces the destination. Replacing a regular file preserves its permissions. A symlink at
    /// the destination is replaced as a link; its target is not followed. On Unix, the parent
    /// directory is synced after replacement on a best-effort basis. Other platforms rely on the
    /// atomic rename and synced file contents; a power loss can still lose the directory-entry
    /// update where the platform does not provide a portable directory sync operation.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or writing the file fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_with_replace(path, |source, destination| {
            std::fs::rename(source, destination)
        })
    }

    fn save_with_replace<F>(&self, path: &Path, replace: F) -> Result<()>
    where
        F: FnOnce(&Path, &Path) -> std::io::Result<()>,
    {
        let contents = self.to_toml()?;
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }

        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let destination_permissions = match std::fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_file() => Some(metadata.permissions()),
            Ok(metadata) if metadata.file_type().is_symlink() => None,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Ok(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "configuration destination must be a regular file or symlink",
                )
                .into());
            }
            Err(error) => return Err(error.into()),
        };
        let (temporary_file, mut file) = create_temporary_file(path, parent)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
        if let Some(permissions) = destination_permissions {
            std::fs::set_permissions(temporary_file.path(), permissions)?;
            file.sync_all()?;
        }
        drop(file);

        // The destination is only touched after the complete temporary file is ready. A failed
        // replace therefore leaves an existing destination unchanged.
        replace(temporary_file.path(), path)?;
        sync_parent_directory(parent);
        Ok(())
    }
}

/// A sibling staging file that is removed if replacement does not consume it.
struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        remove_temporary_file(&self.path);
    }
}

#[cfg(windows)]
fn remove_temporary_file(path: &Path) {
    if std::fs::remove_file(path).is_err() {
        // A copied read-only destination mode can prevent cleanup after a failed rename.
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_readonly(false));
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(not(windows))]
fn remove_temporary_file(path: &Path) {
    let _ = std::fs::remove_file(path);
}

/// Creates a unique staging file beside the destination so replacement stays on one filesystem.
fn create_temporary_file(destination: &Path, parent: &Path) -> Result<(TemporaryFile, File)> {
    let Some(file_name) = destination.file_name() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "configuration destination must name a file",
        )
        .into());
    };

    for _ in 0..128 {
        let id = NEXT_TEMP_FILE_ID.fetch_add(1, Ordering::Relaxed);
        let temporary_path = parent.join(format!(
            ".{}.{}.{}.tmp",
            file_name.to_string_lossy(),
            std::process::id(),
            id
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => {
                return Ok((
                    TemporaryFile {
                        path: temporary_path,
                    },
                    file,
                ));
            }
            Err(error) if error.kind() != std::io::ErrorKind::AlreadyExists => {
                return Err(error.into());
            }
            Err(_) => {}
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "could not allocate a unique configuration staging file",
    )
    .into())
}

/// Syncs the containing directory after replacement where Unix provides this operation.
fn sync_parent_directory(parent: &Path) {
    #[cfg(unix)]
    if let Ok(directory) = File::open(parent) {
        // Replacement has already committed. Directory syncing is best-effort because reporting
        // an error here would imply the prior file survived when it may not have.
        let _ = directory.sync_all();
    }

    #[cfg(not(unix))]
    let _ = parent;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn environment_source(values: &[(&str, &str)]) -> config::Environment {
        let source = values
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect::<config::Map<String, String>>();
        config::Environment::with_prefix(ENV_PREFIX).source(Some(source))
    }

    fn test_directory(label: &str) -> PathBuf {
        static NEXT_TEST_DIR_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "my_lib_config_{label}_{}_{}",
            std::process::id(),
            id
        ))
    }

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

    #[test]
    fn missing_file_yields_defaults() -> Result<()> {
        let path = test_directory("missing").join("absent.toml");
        assert_eq!(
            Config::load_with_environment(Some(&path), environment_source(&[]))?,
            Config::default()
        );
        Ok(())
    }

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

    #[test]
    fn malformed_or_unreadable_file_returns_an_error() -> Result<()> {
        let dir = test_directory("load_error");
        std::fs::create_dir_all(&dir)?;
        let malformed = dir.join("malformed.toml");
        let invalid_type = dir.join("invalid_type.toml");
        std::fs::write(&malformed, "name = [\n")?;
        std::fs::write(&invalid_type, "name = { nested = true }\n")?;
        assert!(Config::load_with_environment(Some(&malformed), environment_source(&[])).is_err());
        assert!(
            Config::load_with_environment(Some(&invalid_type), environment_source(&[])).is_err()
        );
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn failed_replace_preserves_the_previous_file() -> Result<()> {
        let dir = test_directory("replace_failure");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        let original = b"valid prior configuration";
        std::fs::write(&path, original)?;

        let result = Config::new("replacement").save_with_replace(&path, |_, _| {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "simulated replacement failure",
            ))
        });

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path)?, original);
        assert_eq!(std::fs::read_dir(&dir)?.count(), 1);
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn replacing_a_symlink_does_not_modify_its_target() -> Result<()> {
        use std::os::unix::fs::symlink;

        let dir = test_directory("symlink");
        std::fs::create_dir_all(&dir)?;
        let target = dir.join("target.toml");
        let path = dir.join("config.toml");
        std::fs::write(&target, b"target remains untouched")?;
        symlink(&target, &path)?;

        Config::new("replacement").save(&path)?;

        assert!(std::fs::symlink_metadata(&path)?.file_type().is_file());
        assert_eq!(std::fs::read(&target)?, b"target remains untouched");
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn replacing_a_regular_file_preserves_its_permissions() -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let dir = test_directory("permissions");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        std::fs::write(&path, b"old")?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640))?;

        Config::new("replacement").save(&path)?;

        assert_eq!(
            std::fs::metadata(&path)?.permissions().mode() & 0o777,
            0o640
        );
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
