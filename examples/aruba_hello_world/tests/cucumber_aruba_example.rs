use std::process::Command;
use aruba::cucumber_parameters::MaybeNot;
use aruba::text::sanitize_text;
use cucumber::{given, then, Parameter, World, when};

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct ArubaHelloWorld {
    aruba: aruba::prelude::ArubaWorldState
}

/// START COPY FROM ARUBA
///
/// Hello! Please copy the code from the above START COPY comment down to END COPY comment below,
/// and place above the `main` function in a file such as `tests/cucumber_aruba_steps.rs`.
///
/// The unfortunate need to copy code is pending resolution of the following issue:
///   * [cucumber-rs #225: How to write and publish reusable steps?](https://github.com/cucumber-rs/cucumber/issues/235)
///
/// Please be sure to vote and/or comment on that issue as we work towards resolution, thank you!
mod copy_from_aruba {
    use aruba::prelude::*;

    #[derive(Debug, Default, World)]
    pub struct ArubaWorld {
        aruba: ArubaWorldState
    }

    #[when(regex = r"^I run `([^`]*)`$")]
    pub fn aruba_when_run_and_wait(world: &mut ArubaWorld, command: String) {
        world.aruba.when_run_and_wait(command.as_str());
    }
}
/// END COPY FROM ARUBA

// use aruba::step_definitions::run_and_wait_step;
// impl ArubaWorld for ArubaHelloWorld {
//     fn last_command_run(&self) -> &Option<Command> {
//         &None
//     }
// }

// impl Into<&mut ArubaWorld> for ArubaHelloWorld {
//     fn into(&mut self) -> &mut ArubaWorld {
//         &mut self.aruba
//     }
// }


// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ArubaHelloWorld::run("tests/features"));
}