use crate::assertions::{assert_eq_or_ne, assert_str_eq_or_ne};
use crate::prelude::*;
use cucumber::gherkin::Step;
use cucumber::{given, then, when};

#[given(expr = "I prepend {command_line} to PATH")]
#[when(expr = "I prepend {command_line} to PATH")]
pub fn prepend_path_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.prepend_path(command_line.as_ref());
}

#[given(expr = "I{maybe_successfully}run {command_line}{maybe_in_dir}")]
#[when(expr = "I{maybe_successfully}run {command_line}{maybe_in_dir}")]
pub fn run_command_line_step(
    world: &mut ArubaWorld,
    check_exit_status: MaybeSuccessfullyParameter, // Default is *not* to check exit status code
    command_line: CommandLineParameter,
    in_dir: InDirParameter, // Default is to run in a new temp dir
) {
    world.run_command(command_line.as_ref(), in_dir);
    let code = world.last_command_exit_status_code();
    if check_exit_status.into() {
        assert_eq_or_ne(true, code, 0);
    }
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_code_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let code = world.last_command_exit_status_code();
    assert_eq_or_ne(should.into(), code, expected);
}

#[then(expr = "the {output_channel} contains: {string}")]
pub fn output_contains_step(
    world: &mut ArubaWorld,
    channel: OutputChannelParameter,
    expected: String,
) {
    let output = world.last_command_output_string(channel);
    assert_str_contains_or_not(true, sanitize_output(output), sanitize_output(expected));
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
// * Rest of https://github.com/cucumber/aruba/blob/main/lib/aruba/cucumber/command.rb
//   - Run in background
//   - Run interactively
//   - Timeouts
//   - Multiple different commands running
