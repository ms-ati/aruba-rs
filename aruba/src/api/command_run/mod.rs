use lazy_static::lazy_static;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, io};

pub mod existing_or_from_prefix;
pub mod path_or_temp;
pub mod process_state;

pub use existing_or_from_prefix::ExistingOrFromPrefix;
pub use path_or_temp::PathOrTemp;
pub use process_state::ProcessState;

#[derive(Debug)]
pub struct CommandRun {
    pub in_path: PathOrTemp,
    pub command: Command,
    pub process: ProcessState,
}

impl CommandRun {
    pub fn new(command_line: &str, in_path: ExistingOrFromPrefix) -> io::Result<CommandRun> {
        let in_path: PathOrTemp = in_path.try_into()?;

        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(crate::api::text::sanitize_command(command_line))
            .current_dir(&in_path)
            .env("PATH", Self::env_path_prepend_target_dir()?)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let process = ProcessState::Running(command.spawn()?);

        Ok(CommandRun {
            in_path,
            command,
            process,
        })
    }

    pub fn env_path_prepend_target_dir() -> io::Result<OsString> {
        let env_path = env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![Self::find_project_target_dir()?];
        paths.extend(env::split_paths(&env_path));
        env::join_paths(paths.iter())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
    }

    pub fn find_project_target_dir() -> io::Result<PathBuf> {
        let project_root_dir = Self::find_project_root_dir()?;
        Ok(project_root_dir.join("target").join("debug"))
    }

    pub fn find_project_root_dir() -> io::Result<PathBuf> {
        let current_dir = env::current_dir()?;
        let mut path_ancestors = current_dir.ancestors();

        let maybe_project_root_dir = path_ancestors
            .find(|dir| Self::dir_contains_cargo_dot_lock(dir).unwrap_or(false))
            .map(|path| path.to_path_buf());

        maybe_project_root_dir.ok_or_else(|| {
            let err_msg = format!(
                "Cargo.lock not found in paths above: {}",
                current_dir.display()
            );
            io::Error::new(io::ErrorKind::NotFound, err_msg)
        })
    }

    fn dir_contains_cargo_dot_lock(dir: &Path) -> io::Result<bool> {
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
}
