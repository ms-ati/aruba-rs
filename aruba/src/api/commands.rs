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

pub enum ExistingOrMakeTemp {
    ExistingPathOrTemp(PathOrTemp),
    MakeTempWithPrefix(String),
}

impl TryInto<PathOrTemp> for ExistingOrMakeTemp {
    type Error = io::Error;

    fn try_into(self) -> Result<PathOrTemp, Self::Error> {
        match self {
            Self::ExistingPathOrTemp(path_or_temp) => Ok(path_or_temp),
            Self::MakeTempWithPrefix(prefix) => make_temp_dir(prefix),
        }
    }
}

#[derive(Debug)]
pub enum ProcessState {
    Running(Option<Child>),
    Stopped(Output),
}

impl From<Child> for ProcessState {
    fn from(child: Child) -> Self {
        Self::Running(Some(child))
    }
}

impl ProcessState {
    pub fn wait_for_output(&mut self) -> io::Result<&Output> {
        if let ProcessState::Running(maybe_child) = self {
            let child = maybe_child.take().expect("Invalid state: no child process");
            let output = child.wait_with_output()?;
            *self = ProcessState::Stopped(output);
        }

        match self {
            ProcessState::Running(_) => unreachable!(),
            ProcessState::Stopped(ref output) => Ok(output),
        }
    }
}

pub fn run(command_line: &str, in_path: ExistingOrMakeTemp) -> io::Result<CommandRun> {
    let in_path: PathOrTemp = in_path.try_into()?;

    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(crate::api::text::sanitize_command(command_line))
        .current_dir(&in_path)
        .env("PATH", env_path_prepend_target_dir()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let process = ProcessState::Running(Some(command.spawn()?));

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
    use io::{Error, ErrorKind};
    let env_path = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![find_project_target_dir()?];
    paths.extend(env::split_paths(&env_path));
    env::join_paths(paths.iter()).map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
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
