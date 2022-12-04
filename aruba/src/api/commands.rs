use crate::api::PathOrTemp;
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::{env, io};

#[derive(Debug)]
pub struct CommandRun {
    pub in_path: PathOrTemp,
    pub command: Command,
    pub process: ProcessState,
}

pub enum ExistingOrFromPrefix {
    PathOrTemp(PathOrTemp),
    FromPrefix(String),
}

impl TryInto<PathOrTemp> for ExistingOrFromPrefix {
    type Error = io::Error;

    fn try_into(self) -> Result<PathOrTemp, Self::Error> {
        match self {
            Self::PathOrTemp(exists) => Ok(exists),
            Self::FromPrefix(prefix) => make_temp_dir(prefix),
        }
    }
}

#[derive(Debug, Default)]
pub enum ProcessState {
    Running(Child),
    Stopped(Output),
    #[default]
    Unknown,
}

impl ProcessState {
    pub fn wait_for_output(&mut self) -> io::Result<&Output> {
        // See https://stackoverflow.com/questions/68247811/is-there-a-safe-way-to-map-an-enum-variant-to-another-with-just-a-mutable-refere
        match std::mem::replace(self, Self::Unknown) {
            Self::Running(child) => *self = Self::Stopped(child.wait_with_output()?),
            s @ Self::Stopped(_) => *self = s,
            Self::Unknown => return Err(io::Error::new(io::ErrorKind::Other, "Unknown run state")),
        }

        match self {
            ProcessState::Stopped(ref output) => Ok(output),
            _ => unreachable!(),
        }
    }
}

pub fn run(command_line: &str, in_path: ExistingOrFromPrefix) -> io::Result<CommandRun> {
    let in_path: PathOrTemp = in_path.try_into()?;

    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(crate::api::text::sanitize_command(command_line))
        .current_dir(&in_path)
        .env("PATH", env_path_prepend_target_dir()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let process = ProcessState::Running(command.spawn()?);

    Ok(CommandRun {
        in_path,
        command,
        process,
    })
}

pub fn make_temp_dir(prefix: String) -> io::Result<PathOrTemp> {
    let sanitized_prefix = crate::api::text::sanitize_temp_dir(&prefix);
    let temp_dir = tempfile::Builder::new()
        .prefix(&sanitized_prefix)
        .tempdir()?;
    Ok(PathOrTemp::Temp(temp_dir))
}

pub fn env_path_prepend_target_dir() -> io::Result<OsString> {
    let env_path = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![find_project_target_dir()?];
    paths.extend(env::split_paths(&env_path));
    env::join_paths(paths.iter())
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
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
        let err_msg = format!(
            "Cargo.lock not found in paths above: {}",
            current_dir.display()
        );
        io::Error::new(io::ErrorKind::NotFound, err_msg)
    })
}

pub fn dir_contains_cargo_dot_lock(dir: &Path) -> io::Result<bool> {
    lazy_static! {
        static ref CARGO_DOT_LOCK: OsString = OsString::from("Cargo.lock");
    }
    let found_cargo_dot_lock = read_dir(dir)?
        .map(|result| {
            result
                .map(|dir_ent| dir_ent.file_name())
                .unwrap_or_default()
        })
        .any(|os_str| CARGO_DOT_LOCK.eq(&os_str));
    Ok(found_cargo_dot_lock)
}
