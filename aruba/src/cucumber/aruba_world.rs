use crate::api::command_run::{ExistingOrFromPrefix, PathOrTemp};
use crate::api::CommandRun;
use crate::cucumber::{InDirParameter, OutputChannelParameter};
use cucumber::event::ScenarioFinished::StepFailed;
use cucumber::gherkin::Scenario;
use futures::{future, FutureExt};
use std::env;
use std::path::Path;
use std::process::Output;

#[derive(Debug, Default, cucumber::World)]
pub struct ArubaWorld {
    pub paths_to_prepend: Vec<String>,
    pub maybe_last_command_run: Option<CommandRun>,
    pub maybe_scenario_failed: Option<Scenario>,
}

type ArubaDefaultCucumber<I> = cucumber::Cucumber<
    ArubaWorld,
    cucumber::parser::Basic,
    I,
    cucumber::runner::Basic<ArubaWorld>,
    cucumber::writer::Summarize<cucumber::writer::Normalize<ArubaWorld, cucumber::writer::Basic>>,
>;

impl ArubaWorld
where
    ArubaWorld: cucumber::World,
{
    pub fn prepend_path(&mut self, path: &str) {
        self.paths_to_prepend.push(path.to_string())
    }

    pub fn run_command(&mut self, command_line: &str, in_dir: InDirParameter) {
        use InDirParameter::{InCurrDir, InTempDir};

        let in_path = match in_dir {
            InTempDir => {
                let prefix = format!("aruba-run_command-{}", command_line);
                ExistingOrFromPrefix::FromPrefix(prefix)
            }
            InCurrDir => {
                let current = env::current_dir().unwrap_or_default();
                ExistingOrFromPrefix::PathOrTemp(PathOrTemp::Path(current))
            }
        };

        self.maybe_last_command_run =
            Some(CommandRun::new(command_line, in_path, &self.paths_to_prepend).unwrap());
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

    pub fn last_command_output_bytes(&mut self, channel: OutputChannelParameter) -> Vec<u8> {
        match channel {
            OutputChannelParameter::AllOutput => self.last_command_all_output(),
            OutputChannelParameter::Stdout => self.last_command_stdout().to_vec(),
            OutputChannelParameter::Stderr => self.last_command_stderr().to_vec(),
        }
    }

    pub fn last_command_output_string(&mut self, channel: OutputChannelParameter) -> String {
        match String::from_utf8(self.last_command_output_bytes(channel)) {
            Ok(string) => string,
            Err(err) => panic!(
                "Output is not a valid UTF8 string, try as bytes instead: {:?}",
                err.as_bytes()
            ),
        }
    }

    //
    // Call these methods instead of cucumber::World methods: they add the `after` hook to preserve
    // temporary directories in the case of test failure.
    //

    pub fn cucumber<I: AsRef<Path>>() -> ArubaDefaultCucumber<I> {
        cucumber::World::cucumber().after(move |_, _, scenario, event, maybe_world| {
            if let (StepFailed(_, _, _), Some(world)) = (event, maybe_world) {
                world.scenario_failed(scenario);
            }
            future::ready(()).boxed()
        })
    }

    pub async fn run<I: AsRef<Path>>(input: I) {
        Self::cucumber().run_and_exit(input).await;
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
                    "  ^ Preserved temp directory for debugging.\
                   \n    scenario: \"{}\"\
                   \n    command: {:?}\
                   \n    in path: {:?}",
                    scenario.name,
                    command_run.command.get_args().last().unwrap_or_default(),
                    path
                );
            }
        }
    }
}
