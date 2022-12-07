use aruba::prelude::ArubaWorld;
use cucumber::World;
use futures::{future, FutureExt};

fn main() {
    // See also:
    //   - https://cucumber-rs.github.io/cucumber/current/quickstart.html
    //
    futures::executor::block_on(
        ArubaWorld::cucumber()
            .after(move |_, _, scenario, event, maybe_world| {
                use cucumber::event::ScenarioFinished::StepFailed;

                if let (StepFailed(_, _, _), Some(world)) = (event, maybe_world) {
                    world.scenario_failed(scenario);
                }

                future::ready(()).boxed()
            })
            .run_and_exit("tests/features"),
    );
}
