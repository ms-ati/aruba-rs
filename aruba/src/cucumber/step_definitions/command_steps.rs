use crate::assertions::{assert_eq_or_ne, assert_str_eq_or_ne};
use crate::prelude::*;
use cucumber::gherkin::Step;
use cucumber::{then, when};

#[when(expr = "I run {command_line}")]
pub fn run_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.run_command(command_line.as_ref());
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_code_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let code = world.last_command_exit_status_code();
    assert_eq_or_ne(should.into(), code, expected);
}

#[then(expr = "the {output_channel} contains exactly: {string}")]
pub fn output_contains_exactly_step(
    world: &mut ArubaWorld,
    channel: OutputChannelParameter,
    expected: String,
) {
    let output = world.last_command_output_string(channel);
    assert_str_eq_or_ne(true, sanitize_output(output), sanitize_output(expected));
}

#[then(expr = "the {output_channel} contains exactly:")]
pub fn output_contains_exactly_docstring_step(
    world: &mut ArubaWorld,
    channel: OutputChannelParameter,
    step: &Step,
) {
    let expected = step.docstring().map(|s| s.as_str()).unwrap_or_default();
    let output = world.last_command_output_string(channel);
    assert_str_eq_or_ne(true, sanitize_output(output), sanitize_output(expected));
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
