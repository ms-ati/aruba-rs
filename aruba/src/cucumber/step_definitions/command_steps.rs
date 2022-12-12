use crate::prelude::*;
use cucumber::gherkin::Step;
use cucumber::{then, when};
use pretty_assertions::{assert_eq, assert_ne};

#[when(expr = "I run {command_line}")]
pub fn run_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.run_command(command_line.as_ref());
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_code_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let code = world.last_command_exit_status_code();
    if should.into() {
        assert_eq!(code, expected);
    } else {
        assert_ne!(code, expected)
    }
}

#[then(expr = "the {output_channel} contains exactly: {string}")]
pub fn output_contains_exactly_step(
    world: &mut ArubaWorld,
    channel: OutputChannelParameter,
    expected: String,
) {
    match String::from_utf8(world.last_command_output_bytes(channel)) {
        Ok(output) => assert_eq!(output.trim(), expected.trim()),
        Err(error) => assert_eq!(error.into_bytes(), expected.as_bytes()),
    }
}

#[then(expr = "the {output_channel} contains exactly:")]
pub fn output_contains_exactly_docstring_step(
    world: &mut ArubaWorld,
    channel: OutputChannelParameter,
    step: &Step,
) {
    let expected = step.docstring().map(|s| s.as_str()).unwrap_or_default();

    match String::from_utf8(world.last_command_output_bytes(channel)) {
        Ok(output) => assert_eq!(output.trim(), expected.trim()),
        Err(error) => assert_eq!(error.into_bytes(), expected.as_bytes()),
    }
}

// TODO
//
// Convenience:
//   * [Given I build in (release|debug) mode and prepend the target dir to PATH]
//   * [Given I build in '([^']+)' profile and prepend the target dir to PATH]
//   * Given I build and prepend target dir to PATH
//   * Given I build in debug mode and prepend target dir to PATH
//   * Given I build in release mode and prepend target dir to PATH
//   * Given I build in 'release.lto' profile and prepend target dir to PATH
//
// Equivalent to:
//   * Given I run `cargo build`
//       And I prepend `./target/debug` to PATH
//   * Given I run `cargo build --debug`
//       And I prepend `./target/debug` to PATH
//   * Given I run `cargo build --release`
//       And I prepend `./target/release` to PATH
//   * Given I run `cargo build --profile release-lto`
//       And I prepend `./target/release-lto` to PATH
//
//
