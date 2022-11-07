use aruba::prelude::ArubaWorld;
use cucumber::World;

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ArubaWorld::run("tests/features"));
}