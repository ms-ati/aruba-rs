use aruba::prelude::ArubaWorld;
use cucumber::World;
use futures::{future, FutureExt};

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
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
