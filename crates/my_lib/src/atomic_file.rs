//! Atomic file replacement for small application-owned files.

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::error::MyError;
use crate::error::Result;

/// Process-local counter used to avoid collisions between concurrent writes.
static NEXT_TEMP_FILE_ID: AtomicU64 = AtomicU64::new(0);

/// Replaces `path` with `contents` after writing and syncing a sibling staging file.
///
/// Existing regular-file permissions are preserved. A destination symlink is replaced as a link;
/// its target is not followed. On Unix, the containing directory is synced after replacement on a
/// best-effort basis. Other platforms rely on atomic rename and synced file contents, so a power
/// loss can still lose the directory-entry update where portable directory sync is unavailable.
///
/// # Errors
///
/// Returns an error with the destination path and failed operation if staging or replacement
/// fails. The prior destination is left unchanged when failure occurs before replacement.
pub(crate) fn write_atomic(path: &Path, contents: &[u8]) -> Result<()> {
    write_atomic_with(path, contents, |source, destination| {
        std::fs::rename(source, destination)
    })
}

/// Stages a complete file and delegates the final replacement operation.
fn write_atomic_with<F>(path: &Path, contents: &[u8], replace: F) -> Result<()>
where
    F: FnOnce(&Path, &Path) -> std::io::Result<()>,
{
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent).map_err(|source| MyError::AtomicWrite {
        path: path.to_owned(),
        operation: "create parent directories",
        source,
    })?;

    let destination_permissions = match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => Some(metadata.permissions()),
        Ok(metadata) if metadata.file_type().is_symlink() => None,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Ok(_) => {
            return Err(MyError::AtomicWrite {
                path: path.to_owned(),
                operation: "replace destination",
                source: std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "destination must be a regular file or symlink",
                ),
            });
        }
        Err(source) => {
            return Err(MyError::AtomicWrite {
                path: path.to_owned(),
                operation: "inspect destination",
                source,
            });
        }
    };

    let (temporary_file, mut file) =
        create_temporary_file(path, parent).map_err(|source| MyError::AtomicWrite {
            path: path.to_owned(),
            operation: "create staging file",
            source,
        })?;
    file.write_all(contents)
        .map_err(|source| MyError::AtomicWrite {
            path: path.to_owned(),
            operation: "write staging file",
            source,
        })?;
    file.sync_all().map_err(|source| MyError::AtomicWrite {
        path: path.to_owned(),
        operation: "sync staging file",
        source,
    })?;
    if let Some(permissions) = destination_permissions {
        std::fs::set_permissions(temporary_file.path(), permissions).map_err(|source| {
            MyError::AtomicWrite {
                path: path.to_owned(),
                operation: "preserve destination permissions",
                source,
            }
        })?;
        file.sync_all().map_err(|source| MyError::AtomicWrite {
            path: path.to_owned(),
            operation: "sync staging file permissions",
            source,
        })?;
    }
    drop(file);

    // The destination is touched only after the complete staging file is ready.
    replace(temporary_file.path(), path).map_err(|source| MyError::AtomicWrite {
        path: path.to_owned(),
        operation: "replace destination",
        source,
    })?;
    sync_parent_directory(parent);
    Ok(())
}

/// A sibling staging file removed unless replacement consumes it.
struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    /// Returns the staging file path used for writing and replacement.
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryFile {
    /// Removes an unused staging file when the write exits before replacement.
    fn drop(&mut self) {
        remove_temporary_file(&self.path);
    }
}

/// Removes a temporary file, clearing its read-only attribute when necessary.
#[cfg(windows)]
#[expect(
    clippy::permissions_set_readonly_false,
    reason = "Windows cleanup only clears the read-only attribute on the temporary file"
)]
fn remove_temporary_file(path: &Path) {
    if std::fs::remove_file(path).is_err() {
        if let Ok(metadata) = std::fs::metadata(path) {
            let mut permissions = metadata.permissions();
            permissions.set_readonly(false);
            let _ = std::fs::set_permissions(path, permissions);
        }
        let _ = std::fs::remove_file(path);
    }
}

/// Removes a temporary file and ignores cleanup errors during unwinding.
#[cfg(not(windows))]
fn remove_temporary_file(path: &Path) {
    let _ = std::fs::remove_file(path);
}

/// Creates an exclusive sibling file so staging and replacement stay on one filesystem.
fn create_temporary_file(
    destination: &Path,
    parent: &Path,
) -> std::io::Result<(TemporaryFile, File)> {
    let Some(file_name) = destination.file_name() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination must name a file",
        ));
    };

    for _ in 0..128 {
        let id = NEXT_TEMP_FILE_ID.fetch_add(1, Ordering::Relaxed);
        let mut temporary_name = std::ffi::OsString::from(".");
        temporary_name.push(file_name);
        temporary_name.push(format!(".{}.{}.tmp", std::process::id(), id));
        let temporary_path = parent.join(temporary_name);
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
                return Err(error);
            }
            Err(_) => {}
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "could not allocate a unique staging file",
    ))
}

/// Syncs the containing directory after replacement where Unix provides this operation.
fn sync_parent_directory(parent: &Path) {
    #[cfg(unix)]
    if let Ok(directory) = File::open(parent) {
        // Replacement has already committed; an error cannot imply the old file survived.
        let _ = directory.sync_all();
    }

    #[cfg(not(unix))]
    let _ = parent;
}

/// Exercises atomic replacement behavior and temporary-file cleanup.
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;

    #[cfg(unix)]
    use super::write_atomic;
    use super::write_atomic_with;

    /// Temporary test directory removed automatically at the end of each case.
    struct TestDirectory(PathBuf);

    impl TestDirectory {
        /// Creates a uniquely named directory path for one test case.
        fn new(label: &str) -> Self {
            /// Generates unique suffixes for parallel test directories in this process.
            static NEXT_TEST_DIR_ID: AtomicU64 = AtomicU64::new(0);
            let id = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
            Self(std::env::temp_dir().join(format!(
                "my_lib_atomic_{label}_{}_{}",
                std::process::id(),
                id
            )))
        }
    }

    impl Drop for TestDirectory {
        /// Removes the test directory and its contents when the fixture is dropped.
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Leaves the prior destination intact and removes staging data when replacement fails.
    #[test]
    fn failed_replace_preserves_the_previous_file_and_cleans_staging() -> crate::Result<()> {
        let directory = TestDirectory::new("replace_failure");
        std::fs::create_dir_all(&directory.0)?;
        let path = directory.0.join("settings.toml");
        let original = b"prior contents";
        std::fs::write(&path, original)?;

        let result = write_atomic_with(&path, b"replacement", |_, _| {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "simulated replacement failure",
            ))
        });

        let error = result.expect_err("failed replacement should return a contextual error");
        assert!(matches!(
            error,
            crate::MyError::AtomicWrite {
                path: error_path,
                operation: "replace destination",
                ..
            } if error_path == path
        ));
        assert_eq!(std::fs::read(&path)?, original);
        assert_eq!(std::fs::read_dir(&directory.0)?.count(), 1);
        Ok(())
    }

    /// Replaces a symbolic link without writing through to the link target.
    #[cfg(unix)]
    #[test]
    fn replacing_a_symlink_does_not_modify_its_target() -> crate::Result<()> {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new("symlink");
        std::fs::create_dir_all(&directory.0)?;
        let target = directory.0.join("target.txt");
        let path = directory.0.join("settings.txt");
        std::fs::write(&target, b"target remains untouched")?;
        symlink(&target, &path)?;

        write_atomic(&path, b"replacement")?;

        assert!(std::fs::symlink_metadata(&path)?.file_type().is_file());
        assert_eq!(std::fs::read(&target)?, b"target remains untouched");
        Ok(())
    }

    /// Preserves the mode bits of an existing regular destination file.
    #[cfg(unix)]
    #[test]
    fn replacing_a_regular_file_preserves_its_permissions() -> crate::Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let directory = TestDirectory::new("permissions");
        std::fs::create_dir_all(&directory.0)?;
        let path = directory.0.join("settings.txt");
        std::fs::write(&path, b"old")?;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640))?;

        write_atomic(&path, b"new")?;

        assert_eq!(
            std::fs::metadata(&path)?.permissions().mode() & 0o777,
            0o640
        );
        Ok(())
    }
}
