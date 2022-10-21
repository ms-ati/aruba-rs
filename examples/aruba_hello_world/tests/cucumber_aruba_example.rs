use aruba::cucumber_parameters::MaybeNot;
use aruba::text::sanitize_text;
use cucumber::{given, then, Parameter, World};

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct ArubaHelloWorld {
    aruba: ArubaWorldState
}

#[derive(Debug, Default)]
pub struct ArubaWorldState {
    dummy: u32
}

// #[when(regex = r"^I run `([^`]*)`$")]
// fn run_and_wait_step(world: &mut ArubaHelloWorld) {
//     world.aruba.run_and_wait
// }

#[then(expr = "the exit status should{maybe_not} be {int}")]
fn exit_status_step(_world: &mut ArubaHelloWorld, should: MaybeNot, expected: u32) {
    let status = 0;
    if should.into() {
        assert_eq!(status, expected);
    } else {
        assert_ne!(status, expected)
    }

}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ArubaHelloWorld::run("tests/features"));
}