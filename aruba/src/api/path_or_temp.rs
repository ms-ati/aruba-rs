use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// We create temporary directories, which will automatically deleted. When
/// tests fail, we convert them to a `PathBuf`, so that users can inspect the
/// contents after the test run.
#[derive(Debug)]
pub enum PathOrTemp {
    Path(PathBuf),
    Temp(TempDir),
}

/// Safely obtain an `&Path` reference from `TempDir`
impl AsRef<Path> for PathOrTemp {
    fn as_ref(&self) -> &Path {
        match &self {
            PathOrTemp::Path(p) => p.as_ref(),
            PathOrTemp::Temp(t) => t.path(),
        }
    }
}
