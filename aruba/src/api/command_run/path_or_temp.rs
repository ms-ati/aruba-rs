use crate::api::command_run::ExistingOrFromPrefix;
use lazy_static::lazy_static;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// We create temporary directories for each test scenario, which will automatically deleted. When
/// tests fail, we automatically convert `TempDir` to `PathBuf`, so that users can inspect the
/// contents of the temporary directory after the test run.
#[derive(Debug, Default)]
pub enum PathOrTemp {
    Path(PathBuf),
    Temp(TempDir),
    #[default]
    Empty, // Note: used only to release ownership during transition from Temp to Path
}

impl PathOrTemp {
    pub fn new_temp_from_prefix(prefix: &str) -> io::Result<PathOrTemp> {
        let sanitized_prefix = crate::api::text::sanitize_temp_dir(prefix);
        let temp_dir = tempfile::Builder::new()
            .prefix(&sanitized_prefix)
            .tempdir()?;
        Ok(PathOrTemp::Temp(temp_dir))
    }

    pub fn replace_temp_with_path(&mut self) {
        // See https://stackoverflow.com/questions/68247811/is-there-a-safe-way-to-map-an-enum-variant-to-another-with-just-a-mutable-refere
        match std::mem::replace(self, Self::Empty) {
            Self::Path(p) => *self = Self::Path(p),
            Self::Temp(t) => *self = Self::Path(t.into_path()),
            Self::Empty => (),
        }
    }
}

impl TryFrom<ExistingOrFromPrefix> for PathOrTemp {
    type Error = io::Error;

    fn try_from(value: ExistingOrFromPrefix) -> Result<Self, Self::Error> {
        match value {
            ExistingOrFromPrefix::PathOrTemp(p) => Ok(p),
            ExistingOrFromPrefix::FromPrefix(p) => PathOrTemp::new_temp_from_prefix(&p),
        }
    }
}

/// Safely obtain an `&Path` reference from `TempDir`
impl AsRef<Path> for PathOrTemp {
    fn as_ref(&self) -> &Path {
        lazy_static! {
            static ref EMPTY_PATH: PathBuf = PathBuf::default();
        }

        match &self {
            Self::Path(p) => p.as_ref(),
            Self::Temp(t) => t.as_ref(),
            Self::Empty => EMPTY_PATH.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_deletes_on_drop() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        assert!(path.exists());
        {
            let _ = PathOrTemp::Temp(temp_dir);
            // Drop of PathOrTemp::Temp should occur here, causing deletion of path
        }
        assert!(!path.exists());
    }

    #[test]
    fn replace_temp_with_path_does_not_delete_on_drop() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();
        {
            let mut path_or_temp = PathOrTemp::Temp(temp_dir);
            path_or_temp.replace_temp_with_path();
            // Drop of PathOrTemp::Path should occur here, does *not* delete path
        }
        assert!(path.exists());
    }
}
