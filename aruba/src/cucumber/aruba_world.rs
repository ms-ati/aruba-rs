use crate::api::command_run::{ExistingOrFromPrefix, PathOrTemp};
use crate::api::CommandRun;
use cucumber::gherkin::Scenario;
use std::process::Output;

#[derive(Debug, Default, cucumber::World)]
pub struct ArubaWorld {
    pub maybe_last_command_run: Option<CommandRun>,
    pub maybe_scenario_failed: Option<Scenario>,
}

impl ArubaWorld {
    pub fn run_command(&mut self, command_line: &str) {
        let in_temp_dir = ExistingOrFromPrefix::FromPrefix("".to_string());
        self.maybe_last_command_run = Some(CommandRun::new(command_line, in_temp_dir).unwrap());
    }

    pub fn scenario_failed(&mut self, scenario: &Scenario) {
        self.maybe_scenario_failed = Some(scenario.clone());
        if let Some(command_run) = self.maybe_last_command_run.as_mut() {
            command_run.keep_temp_path();
        }
    }

    pub fn last_command(&mut self) -> &mut CommandRun {
        self.maybe_last_command_run
            .as_mut()
            .expect("No command has been run")
    }

    pub fn last_command_output(&mut self) -> &Output {
        self.last_command().process.wait_for_output().unwrap()
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

impl Drop for ArubaWorld {
    fn drop(&mut self) {
        if let (Some(scenario), Some(command_run)) = (
            self.maybe_scenario_failed.as_ref(),
            self.maybe_last_command_run.as_ref(),
        ) {
            if let PathOrTemp::Path(path) = &command_run.in_path {
                println!(
                    "  ^ Preserved temp directory for debugging.\n    scenario: \"{}\"\n    command: {:?}\n    in path: {:?}",
                    scenario.name,
                    command_run.command.get_args().last().unwrap_or_default(),
                    path
                );
            }
        }
    }
}
