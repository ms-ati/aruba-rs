use cucumber::{given, then, when};
use crate::prelude::*;

#[when(expr = "I run {command_line}")]
pub fn run_and_wait_step(world: &mut ArubaWorld, command_line: CommandLineParameter) {
    world.run_and_wait(&command_line.as_ref());
}

#[then(expr = "the exit status code should{maybe_not}be {int}")]
pub fn exit_status_step(world: &mut ArubaWorld, should: MaybeNotParameter, expected: i32) {
    let status = world.last_command_exit_status();
    let code = status.code().unwrap_or_else(|| panic!("Exit status without code: {}", &status));

    if should.into() {
        assert_eq!(code, expected);
    } else {
        assert_ne!(code, expected)
    }
}

// #[then(expr = "the exit status should{maybe_not}be {word}")]
// pub fn exit_status_step(_world: &mut ArubaWorld, should: MaybeNotParameter, expected: String) {
//     let status = 0;
//     if should.into() {
//         assert_eq!(status.to_string(), expected);
//     } else {
//         assert_ne!(status.to_string(), expected)
//     }
// }

// #[then(expr = "the exit status should be {int}")]
// pub fn exit_status_step(_world: &mut ArubaWorld, expected: u32) {
//     let status = 0;
//     assert_eq!(status, expected);
// }

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