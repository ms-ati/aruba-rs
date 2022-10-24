// use cucumber::{then, when, World};
// use crate::parameters::MaybeNot;
// use crate::cucumber_world::{ArubaWorld, ArubaWorldState};

// TODO: Make this the SOURCE of the COPY block in the example test

// #[when(regex = r"^I run `([^`]*)`$")]
// pub fn run_and_wait_step(world: &mut ArubaWorldState) {
//     //world.aruba.run_and_wait
// }
//
// #[then(expr = "the exit status should{maybe_not} be {int}")]
// pub fn exit_status_step(_world: &mut ArubaWorldState, should: MaybeNot, expected: u32) {
//     let status = 0;
//     if should.into() {
//         assert_eq!(status, expected);
//     } else {
//         assert_ne!(status, expected)
//     }
// }