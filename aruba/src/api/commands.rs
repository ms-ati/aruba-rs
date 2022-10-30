use std::{env, io};
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

use lazy_static::lazy_static;
use tempfile::TempDir;

#[derive(Debug)]
pub struct ChildInPath {
    pub child: Child,
    pub path_or_temp: PathOrTemp,
}

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
            PathOrTemp::Temp(t) => t.path()
        }
    }
}

pub enum ExistingOrMakeTemp {
    ExistingPathOrTemp(PathOrTemp),
    MakeTempWithPrefix(String)
}

impl TryInto<PathOrTemp> for ExistingOrMakeTemp {
    type Error = io::Error;

    fn try_into(self) -> Result<PathOrTemp, Self::Error> {
        match self {
            Self::ExistingPathOrTemp(path_or_temp) => Ok(path_or_temp),
            Self::MakeTempWithPrefix(prefix)       => make_temp_dir(prefix)
        }
    }
}

pub fn run_and_wait(command_line: &str, in_path: ExistingOrMakeTemp) -> io::Result<ChildInPath> {
    let mut child_in_path = run(command_line, in_path)?;
    let _ = child_in_path.child.wait()?;
    Ok(child_in_path)
}

pub fn run(command_line: &str, in_path: ExistingOrMakeTemp) -> io::Result<ChildInPath> {
    let path_or_temp = in_path.try_into()?;

    // path env

    let child = Command::new("sh")
        .arg("-c")
        .arg(text::sanitize_command(command_line))
        .current_dir(&path_or_temp)
        .env("PATH", env_path_prepend_target_dir()?)
        .spawn()?;

    Ok(ChildInPath { child, path_or_temp })
}

pub fn make_temp_dir(prefix: String) -> io::Result<PathOrTemp> {
    let sanitized_prefix = text::sanitize_temp_dir(&prefix);
    let tmp_dir = tempfile::Builder::new()
        .prefix(&sanitized_prefix)
        .tempdir()?;
    Ok(PathOrTemp::Temp(tmp_dir))
}

pub fn env_path_prepend_target_dir() -> io::Result<OsString> {
    use io::{Error, ErrorKind};
    let env_path = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![find_project_target_dir()?];
    paths.extend(env::split_paths(&env_path));
    env::join_paths(paths.iter()).
        map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
}

pub fn find_project_target_dir() -> io::Result<PathBuf> {
    let project_root_dir = find_project_root_dir()?;
    Ok(project_root_dir.join("target").join("debug"))
}

pub fn find_project_root_dir() -> io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let mut path_ancestors = current_dir.ancestors();

    let maybe_project_root_dir = path_ancestors
        .find(|dir| dir_contains_cargo_dot_lock(dir).unwrap_or(false))
        .map(|path| path.to_path_buf());

    maybe_project_root_dir.ok_or_else(|| {
        let err_msg = format!("Cargo.lock not found in paths above: {}", current_dir.display());
        io::Error::new(io::ErrorKind::NotFound, err_msg)
    })
}

pub fn dir_contains_cargo_dot_lock(dir: &Path) -> io::Result<bool> {
    lazy_static! { static ref CARGO_DOT_LOCK: OsString = OsString::from("Cargo.lock"); }
    let found_cargo_dot_lock = read_dir(dir)?
        .map(|result| result.map(|dir_ent| dir_ent.file_name()).unwrap_or_default())
        .any(|os_str| CARGO_DOT_LOCK.eq(&os_str));
    Ok(found_cargo_dot_lock)
}

mod text {
    use lazy_static::lazy_static;
    use regex::Regex;

    pub fn sanitize_command(text: &str) -> String {
        //DO WE NEED THESE?
        //let text = unescape_text(text);
        //let text = extract_text(text) if aruba.config.remove_ansi_escape_sequences
        text.trim().to_owned()
    }

    pub fn sanitize_temp_dir(prefix: &str) -> String {
        lazy_static! { static ref RE_NON_WORDS: Regex = Regex::new(r"\W").unwrap(); }
        RE_NON_WORDS.replace_all(prefix.to_lowercase().as_str(), "-").to_string()
    }
}
