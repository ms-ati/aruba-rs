use std::process::ExitStatus;
use crate::api::commands::{run_and_wait, ChildInPath, ExistingOrMakeTemp};

#[derive(Debug, Default, cucumber::World)]
pub struct ArubaWorld {
    last_command_run: Option<ChildInPath>
}

impl ArubaWorld {
    pub fn run_and_wait(&mut self, command_line: &str) {
        let in_path = ExistingOrMakeTemp::MakeTempWithPrefix("".to_string());
        let child_in_path = run_and_wait(command_line, in_path).unwrap();
        self.last_command_run = Some(child_in_path);
    }

    pub fn last_command_exit_status(&mut self) -> ExitStatus {
        let child_in_path = &mut self.last_command_run.as_mut().expect("No command has been run");
        let child = &mut child_in_path.child;
        child.wait().unwrap()
    }
}
