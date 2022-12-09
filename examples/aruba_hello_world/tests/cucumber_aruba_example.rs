use aruba::prelude::ArubaWorld;

fn main() {
    //
    // See also:
    //   - https://cucumber-rs.github.io/cucumber/current/quickstart.html
    //
    futures::executor::block_on(ArubaWorld::run("tests/features"));
}
