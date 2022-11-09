use std::str::FromStr;
use bstr::ByteSlice;
use cucumber::{then, when, World};
use cucumber::gherkin::Step;
use pretty_assertions::{assert_eq, assert_ne};
use crate::api::text::trim_prefix_single_newline;
use crate::prelude::*;

#[when(expr = "I run {command_line}")]
pub fn run_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.run_command(command_line.as_ref());
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_code_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let code = world.last_command_exit_status_code();
    if should.into() { assert_eq!(code, expected); } else { assert_ne!(code, expected) }
}

#[then(expr = "the {output_channel} contains exactly: {string}")]
pub fn output_contains_step(world: &mut ArubaWorld, channel: OutputChannelParameter, expected: String) {
    let bytes = match channel {
        OutputChannelParameter::AllOutput => world.last_command_all_output(),
        OutputChannelParameter::Stdout    => world.last_command_stdout().to_vec(),
        OutputChannelParameter::Stderr    => world.last_command_stderr().to_vec(),
    };

    match String::from_utf8(bytes) {
        Ok(output) => assert_eq!(output, expected),
        Err(error) => {
            let output_bytes = error.into_bytes();
            assert!(output_bytes.contains_str(&expected));
        },
    }
}

#[then(regex = r"^the (output|stdout|stderr) contains exactly:$")]
pub fn output_contains_docstring_step(world: &mut ArubaWorld, channel_string: String, step: &Step) {
    let channel = OutputChannelParameter::from_str(&channel_string).unwrap();

    let bytes = match channel {
        OutputChannelParameter::AllOutput => world.last_command_all_output(),
        OutputChannelParameter::Stdout    => world.last_command_stdout().to_vec(),
        OutputChannelParameter::Stderr    => world.last_command_stderr().to_vec(),
    };

    let expected = step.docstring()
        .map(trim_prefix_single_newline) // WORKAROUND: Rust Gherkin doc strings start w/ new line
        .unwrap_or_default();

    match String::from_utf8(bytes) {
        Ok(output) => assert_eq!(output, expected),
        Err(error) => {
            let output_bytes = error.into_bytes();
            assert!(output_bytes.contains_str(&expected));
        },
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