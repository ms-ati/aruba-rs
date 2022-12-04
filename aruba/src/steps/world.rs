use crate::api::commands::{run, CommandRun, ExistingOrFromPrefix};
use std::process::Output;

#[derive(Debug, Default, cucumber::World)]
pub struct ArubaWorld {
    pub last_command_run: Option<CommandRun>,
}

impl ArubaWorld {
    pub fn run_command(&mut self, command_line: &str) {
        let in_temp_dir = ExistingOrFromPrefix::FromPrefix("".to_string());
        self.last_command_run = Some(run(command_line, in_temp_dir).unwrap());
    }

    pub fn last_command_output(&mut self) -> &Output {
        let command_run = self
            .last_command_run
            .as_mut()
            .expect("No command has been run");
        command_run.process.wait_for_output().unwrap()
    }

    pub fn last_command_exit_status_code(&mut self) -> i32 {
        let status = &self.last_command_output().status;
        status
            .code()
            .unwrap_or_else(|| panic!("Exit status without code: {:?}", status))
    }

    pub fn last_command_stdout(&mut self) -> &Vec<u8> {
        &self.last_command_output().stdout
    }

    pub fn last_command_stderr(&mut self) -> &Vec<u8> {
        &self.last_command_output().stderr
    }

    pub fn last_command_all_output(&mut self) -> Vec<u8> {
        let mut all = self.last_command_stdout().clone();
        all.extend(self.last_command_stderr());
        all
    }
}
