use bstr::ByteSlice;
use cucumber::{then, when};
use crate::prelude::*;

#[when(expr = "I run {command_line}")]
pub fn run_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.run_command(command_line.as_ref());
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let status = world.last_command_exit_status();
    let code = status.code().unwrap_or_else(|| panic!("Exit status without code: {:?}", &status));
    if should.into() { assert_eq!(code, expected); } else { assert_ne!(code, expected) }
}

#[then(expr = "the {output_channel} contains: {string}")]
pub fn output_contains_step(world: &mut ArubaWorld, channel: OutputChannelParameter, expected: String) {
    let bytes = match channel {
        OutputChannelParameter::AllOutput => world.last_command_all_output(),
        OutputChannelParameter::Stdout    => world.last_command_stdout().to_vec(),
        OutputChannelParameter::Stderr    => world.last_command_stderr().to_vec(),
    };

    match String::from_utf8(bytes) {
        Ok(output) => assert!(output.contains(&expected), "\n\"\"\"\n{}\"\"\"", output),
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